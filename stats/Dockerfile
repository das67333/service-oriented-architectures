FROM python:3.12-slim

WORKDIR /app
RUN pip install --no-cache-dir --upgrade fastapi uvicorn[standard] clickhouse-connect
COPY main.py .
CMD ["uvicorn", "main:app" , "--host", "0.0.0.0", "--port", "3000"]
