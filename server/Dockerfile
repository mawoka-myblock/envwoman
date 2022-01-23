FROM tiangolo/uvicorn-gunicorn-fastapi:python3.9-slim
WORKDIR /app/
COPY ./helpers /app/helpers/
COPY ./routers /app/routers/
COPY *.* /app/
COPY Pipfile* /app/
RUN pip install pipenv && pipenv install --system
EXPOSE 80