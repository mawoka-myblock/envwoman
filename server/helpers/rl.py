from slowapi.util import get_remote_address
from helpers.config import settings

from slowapi import Limiter


limiter = Limiter(key_func=get_remote_address, storage_uri=settings.redis)
