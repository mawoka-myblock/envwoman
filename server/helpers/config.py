from pydantic import BaseSettings, RedisDsn
from pymongo.collection import Collection
from motor.motor_asyncio import AsyncIOMotorClient
import aioredis


def col(column: str) -> Collection:
    return AsyncIOMotorClient(settings.mongo_url)[settings.mongo_db][column]


def redis() -> aioredis.client.Redis:
    aioredis.decode_responses = True
    return aioredis.from_url(settings.redis, encoding="utf-8")


class Settings(BaseSettings):
    """
    Settings class for the shop app.
    """
    root_address: str = "http://127.0.0.1:8000"
    # redis: RedisDsn = "redis://localhost:6379/0"
    skip_email_verification: bool = False
    redis: RedisDsn = "redis://localhost:6379/0"
    mongo_url: str
    mongo_db: str = "envwoman"
    mail_address: str
    mail_password: str
    mail_username: str
    mail_server: str
    mail_port: int
    secret_key: str

    # access_token_expire_minutes: int = 30
    # cache_expiry: int = 86400

    class Config:
        env_file = ".env"
        env_file_encoding = 'utf-8'


settings = Settings()
