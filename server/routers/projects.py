import copy
from datetime import datetime

from fastapi import APIRouter, HTTPException, status, Header

from helpers.auth import *

from pymongo import UpdateOne, ReturnDocument

router = APIRouter()


@router.get("/list", response_model=List[BaseProject], response_model_exclude={"data"})
async def get_projects(mawoka_auth_header: Optional[str] = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    return await col("projects").find({"owner": user.email}).to_list(length=None)


@router.post("/create", response_model=BaseProject)
async def create_project(project: CreateProject, mawoka_auth_header: Optional[str] = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    if await col("projects").find_one({"name": project.name}) is not None:
        raise HTTPException(status_code=status.HTTP_409_CONFLICT, detail="Project already exists")
    create_proj_dict = project.dict(by_alias=True)
    now = str(datetime.now().replace(microsecond=0).isoformat())
    create_proj_dict.update({"data": []})
    for environment in create_proj_dict["environments"]:
        if create_proj_dict["selected_environment"] == environment:
            create_proj_dict["data"].append({environment: project.data})
        else:
            create_proj_dict["data"].append({environment: ""})
    create_proj_dict.update({"owner": user.email, "date_created": now, "date_modified": now})
    projindb = ProjectInDB(**create_proj_dict)
    await col("projects").insert_one(projindb.dict(by_alias=True))
    return ProjectInDB(**projindb.dict())


@router.get("/get/{project_name}", response_model=BaseProject)
async def get_project(project_name: str, mawoka_auth_header: Optional[str] = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    project = await col("projects").find_one({"name": project_name, "owner": user.email})
    if project is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Project not found")
    return project


@router.post("/update/{project_name}", response_model=BaseProject)
async def update_prject(project_name: str, project: UpdateProject, mawoka_auth_header: Optional[str] = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    update_dict = project.dict(by_alias=True, exclude={"data", "selected_environment"})
    keys = copy.deepcopy(update_dict)
    for i in keys:
        if update_dict[i] is None:
            update_dict.pop(i)

    # result = await col("projects").bulk_write([
    #     UpdateOne({"name": project_name, "owner": user.email}, {"$set": update_dict}),
    #     UpdateOne({"name": project_name, "owner": user.email},
    #               {"$addToSet": {"data": project.selected_environment, "environments": project.selected_environment}}),
    #     UpdateOne({"name": project_name, "owner": user.email}, {
    #         "$set": {f"data.{project.selected_environment}": project.data,
    #                  "date_modified": str(datetime.now().replace(microsecond=0).isoformat())}}),
    # ])
    update = await col("projects").update_one({"name": project_name, "owner": user.email}, {"$set": update_dict})
    update_set = await col("projects").update_one({"name": project_name, "owner": user.email}, {
        "$addToSet": {"environments": project.selected_environment,
                      }})
    res = await col("projects").find_one_and_update({"name": project_name, "owner": user.email},
                                                    {"$set": {f"data.$": {project.selected_environment: project.data}},
                                                     # "date_modified": str(
                                                     #     datetime.now().replace(microsecond=0).isoformat())
                                                     })
    # res = await col("projects").find_one_and_update({"name": project_name, "owner": user.email},
    #                                                 {"$and": [{"$addToSet": {"data": project.selected_environment}}, {
    #                                                     "$set": {f"data.{project.selected_environment}": project.data}},
    #                                                           {"$set": update_dict}, {"addToSet": {
    #                                                         "environments": project.selected_environment}}]})
    # res = await col("projects").find_one_and_update({"name": project_name, "owner": user.email},
    # {"$set": update_dict})
    if res is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Project not found")
    return BaseProject(**res)


@router.delete("/delete/{project_name}")
async def delete_project(project_name: str, mawoka_auth_header: Optional[str] = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    if (await col("projects").delete_one({"name": project_name, "owner": user.email})).deleted_count == 0:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Project not found")
