from fastapi import FastAPI
from routers import users, cli_login, projects

app = FastAPI()


app.include_router(users.router, prefix="/api/v1/users", tags=["users"])
app.include_router(cli_login.router, prefix="/api/v1/cli-login", tags=["cli"])
app.include_router(projects.router, prefix="/api/v1/projects", tags=["projects"])
