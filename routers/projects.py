from fastapi.responses import RedirectResponse, PlainTextResponse, JSONResponse
from helpers.security.verify import send_mail
from datetime import datetime

from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks, Header
import copy
from helpers.config import col, redis
from typing import List, Union
import os
from helpers.config import settings
from helpers.models import *
from helpers.auth import *

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
    create_proj_dict = project.dict()
    now = str(datetime.now().replace(microsecond=0).isoformat())
    create_proj_dict.update({"owner": user.email, "date_created": now, "date_modified": now})
    projindb = ProjectInDB(**create_proj_dict)
    await col("projects").insert_one(projindb.dict())
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
    update_dict = project.dict()
    keys = copy.deepcopy(update_dict)
    for i in keys:
        if update_dict[i] is None:
            update_dict.pop(i)
    res = await col("projects").find_one_and_update({"name": project_name, "owner": user.email}, {"$set": update_dict})
    return BaseProject(**res)
