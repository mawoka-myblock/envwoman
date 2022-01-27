import smtplib
import ssl
from datetime import datetime
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

import aiohttp
from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks, Request
from fastapi.responses import RedirectResponse, PlainTextResponse, JSONResponse
from helpers.security.verify import send_mail

from helpers.config import col, redis
from typing import List, Union
import os
from helpers.config import settings
from helpers.models import *
from helpers.rl import limiter
from helpers.auth import *

router = APIRouter()


async def send_login_notification(email: str, request: Request):
    """
    Send a login notification to the user.
    """
    ip_data = None
    async with aiohttp.ClientSession() as session:
        async with session.post(f"https://api.techniknews.net/ipgeo/{request.client.host}") as resp:
            ip_data = await resp.json()
    msg = MIMEMultipart('alternative')
    msg['Subject'] = "Verify your account"
    msg['From'] = settings.mail_address
    msg['To'] = email

    text = f"""
            Hey {email}!
            Somebody logged in successfully with your account. The IP address was {request.client.host}. It was 
            located in {ip_data['city']}, {ip_data['regionName']}, {ip_data['country']}. I hope it was you! If not, 
            please contact me as soon as possible. 
            \nCheers! """
    msg.attach(MIMEText(text, "plain"))
    context = ssl.SSLContext(ssl.PROTOCOL_TLS)
    server = smtplib.SMTP(host=settings.mail_server, port=settings.mail_port)
    server.ehlo()
    server.starttls(context=context)
    server.ehlo()
    server.login(settings.mail_username, settings.mail_password)
    server.sendmail(settings.mail_address, email, msg.as_string())


@router.get("/token/{token}", response_class=PlainTextResponse)
@limiter.limit("1/minute")
async def get_token(token: str, request: Request, background_tasks: BackgroundTasks):
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
@limiter.limit("1/minute")
async def test_token(key: str, request: Request):
    """
    Test if a key is valid.
    """
    user = await get_user_from_header(key)
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid key")
