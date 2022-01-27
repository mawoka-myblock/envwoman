from datetime import datetime

import aiohttp
from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks, Header, Body
from fastapi.responses import RedirectResponse, PlainTextResponse, JSONResponse
from helpers.security.verify import send_mail

from helpers.config import col, redis
from typing import List, Union
from helpers.config import settings
import json
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
        return RedirectResponse("https://envwoman.mawoka.eu/success", 306)
    else:
        return PlainTextResponse("Wrong Code!", 404)


@router.post("/create", response_model=BaseUser, status_code=status.HTTP_201_CREATED)
async def create_user(user: BaseUser, background_task: BackgroundTasks, h_captcha_response: str = Header(None)) -> \
        Union[BaseUser, JSONResponse]:
    async with aiohttp.ClientSession() as session:
        async with session.post("https://hcaptcha.com/siteverify",
                                data={"response": h_captcha_response, "secret": settings.hcaptcha_key}) as resp:
            data = await resp.json()
            if not data["success"]:
                print(data)
                raise HTTPException(status_code=400, detail="Invalid captcha")

    if not data["success"]:
        raise HTTPException(status_code=400, detail="Invalid captcha")
    if await col("users").find_one({"email": user.email}) is not None:
        raise HTTPException(status_code=400, detail="User already registered")
    user.password = get_password_hash(user.password)
    if len(user.email) >= 32:
        return JSONResponse({"details": "Username mustn't be 32 characters long"}, 400)
    userindb = UserInDB(**user.dict(by_alias=True), date_joined=str(datetime.now()))
    _id = await col("users").insert_one(userindb.dict(by_alias=True))
    background_task.add_task(send_mail, email=user.email)
    return JSONResponse({"details": "User created successfully"}, status_code=201)


@router.post("/login", status_code=status.HTTP_200_OK, response_class=PlainTextResponse)
async def login(user: BaseUser, h_captcha_response: str = Header(None)):
    """
    Logs in a user
    :param user: The user object
    :param h_captcha_response: The hcaptcha code
    :return: The token
    """
    async with aiohttp.ClientSession() as session:
        async with session.post("https://hcaptcha.com/siteverify",
                                data={"response": h_captcha_response, "secret": settings.hcaptcha_key}) as resp:
            data = await resp.json()
            if not data["success"]:
                print(data)
                raise HTTPException(status_code=400, detail="Invalid captcha")
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


@router.post("/logout", status_code=status.HTTP_200_OK, response_class=PlainTextResponse)
async def logout(api_key: str, mawoka_auth_header: str = Header(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    await col("users").find_one_and_update({"api_key": api_key}, {"$set": {"api_keys": []}})
    return "Logged out"


@router.delete("/delete", status_code=status.HTTP_200_OK, response_class=PlainTextResponse)
async def delete_user(api_key: str, mawoka_auth_header: str = Header(None), password: str = Body(None)):
    user = await get_user_from_header(mawoka_auth_header)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid authentication credentials")
    if await col("users").find_one_and_delete({"api_key": api_key, "password": get_password_hash(password)}) is None:
        raise HTTPException(status_code=400, detail="Invalid password or api_key")
    await col("projects").delete_many({"owner": user.email})
    return "User deleted"
