from passlib.context import CryptContext
from typing import Optional, Union
from helpers.config import col
from helpers.models import *

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


async def get_user_from_email(email: str):
    pass


def verify_password(plain_password, hashed_password):
    return pwd_context.verify(plain_password, hashed_password)


def get_password_hash(password):
    return pwd_context.hash(password)


async def get_user_from_header(header) -> Union[None, UserInDB]:
    res = await col("users").find_one({"api_keys": header, "verified": True})
    if res is None:
        return None
    else:
        return UserInDB(**res)
