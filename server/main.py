from fastapi import FastAPI
from routers import users, cli_login, projects
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()


origins = [
    "https://envwoman.mawoka.eu.org",
    "https://envwoman.mawoka.eu",
    "http://localhost",
    "http://localhost:8080",
]
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


app.include_router(users.router, prefix="/api/v1/users", tags=["users"])
app.include_router(cli_login.router, prefix="/api/v1/cli-login", tags=["cli"])
app.include_router(projects.router, prefix="/api/v1/projects", tags=["projects"])
