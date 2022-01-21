from typing import Optional, List, Union, Dict
from bson import ObjectId
from pydantic import Field, validator
from pydantic import BaseModel as PydanticBaseModel


class BaseModel(PydanticBaseModel):
    class Config:
        allow_population_by_field_name = True
        arbitrary_types_allowed = True
        json_encoders = {
            ObjectId: str
        }


class PyObjectId(ObjectId):
    @classmethod
    def __get_validators__(cls):
        yield cls.validate

    @classmethod
    def validate(cls, v):
        if not ObjectId.is_valid(v):
            raise ValueError("Invalid objectid")
        return ObjectId(v)

    @classmethod
    def __modify_schema__(cls, field_schema):
        field_schema.update(type="string")


class BaseUser(BaseModel):
    password: str
    email: str

    @validator('email')
    def shorter_than_32_chars(cls, v):
        if len(v) >= 32:
            raise ValueError('must contain a space')
        return v


class UserInDB(BaseUser):
    id: Optional[PyObjectId] = Field(default_factory=PyObjectId, alias="_id")
    date_joined: str
    verified: Union[bool, str] = False
    api_keys: List[str] = []


class CreateProject(BaseModel):
    name: str
    description: Optional[str] = ""
    #    owner: str
    members: List[str] = []
    #    date_created: str
    #    date_modified: str
    environments: List[str] = ["standard"]
    selected_environment: str = "standard"
    data: str


class BaseProject(CreateProject):
    owner: str
    date_created: str
    date_modified: str
    data: List[Dict[str, str]] = [{"standard": ""}]


class UpdateProject(BaseModel):
    members: Optional[List[str]] = None
    description: Optional[str] = None
    data: Optional[str] = None
    environments: Optional[List[str]] = None
    selected_environment: Optional[str] = "standard"


class ProjectInDB(BaseProject):
    id: Optional[PyObjectId] = Field(default_factory=PyObjectId, alias="_id")
