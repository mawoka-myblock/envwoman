from datetime import datetime

import aiohttp
from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks
from fastapi.responses import RedirectResponse, PlainTextResponse, JSONResponse
from helpers.security.verify import send_mail

from helpers.config import col, redis
from typing import List, Union
from helpers.config import settings
from helpers.models import *
import random
from helpers.auth import *

router = APIRouter()


@router.get("/users/verify/{key}")
async def get_verify_code(key: str):
    """
    Needs to be called from the email link
    :param key: The verification key from the email
    :return: Redirect to the login page
    """
    res = await col("users").find_one_and_update({"verified": key}, {"$set": {"verified": True}})
    if res is not None:
        return RedirectResponse(settings.root_address, 306)
    else:
        return PlainTextResponse("Wrong Code!", 404)


@router.post("/create", response_model=BaseUser, status_code=status.HTTP_201_CREATED)
async def create_user(user: BaseUser, background_task: BackgroundTasks) -> Union[BaseUser, JSONResponse]:

    if await col("users").find_one({"email": user.email}) is not None:
        raise HTTPException(status_code=400, detail="User already registered")
    user.password = get_password_hash(user.password)
    if len(user.email) >= 32:
        return JSONResponse({"details": "Username mustn't be 32 characters long"}, 400)
    userindb = UserInDB(**user.dict(by_alias=True), date_joined=str(datetime.now()))
    _id = await col("users").insert_one(userindb.dict(by_alias=True))
    background_task.add_task(send_mail, email=user.email)


@router.post("/login", status_code=status.HTTP_200_OK, response_class=PlainTextResponse)
async def login(user: BaseUser):
    """
    Logs in a user
    :param user: The user object
    :return: The token
    """
    userindb = await col("users").find_one({"email": user.email, "verified": True})
    if userindb is None:
        raise HTTPException(status_code=400, detail="User not found")
    if not verify_password(user.password, userindb["password"]):
        raise HTTPException(status_code=400, detail="Wrong password")
    if not userindb["verified"]:
        raise HTTPException(status_code=400, detail="User not verified")
    code = random.randint(0000000000, 9999999999)
    await redis().set(name=f"token-{str(code)}", value=str(userindb["_id"]), ex=600)
    return str(code)
