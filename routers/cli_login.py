from datetime import datetime

from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks
from fastapi.responses import RedirectResponse, PlainTextResponse, JSONResponse
from helpers.security.verify import send_mail

from helpers.config import col, redis
from typing import List, Union
import os
from helpers.config import settings
from helpers.models import *
from helpers.auth import *

router = APIRouter()


@router.get("/token/{token}", response_class=PlainTextResponse)
async def get_token(token: str):
    """
    Get a token for a user.
    """
    redis_res = await redis().get(f"token-{token}")
    if redis_res is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Token not found")
    api_key = os.urandom(32).hex()
    await col("users").update_one({"_id": ObjectId(redis_res.decode('utf-8'))},
                                  {"$addToSet": {"api_keys": api_key}})
    return api_key


@router.get("/test-key/{key}", response_class=PlainTextResponse)
async def test_token(key: str):
    """
    Test if a key is valid.
    """
    user = await get_user_from_header(key)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid key")

