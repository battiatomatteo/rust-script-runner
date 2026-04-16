@echo off
echo Creazione struttura backend FastAPI...

REM === CREA CARTELLE ===
mkdir backend
cd backend
mkdir app
mkdir app\routers
mkdir app\models
mkdir app\schemas
mkdir app\services

REM === main.py ===
echo from fastapi import FastAPI> app\main.py
echo from app.routers import users, notes, comments>> app\main.py
echo.>> app\main.py
echo app = FastAPI(title="MindChat Backend")>> app\main.py
echo.>> app\main.py
echo app.include_router(users.router)>> app\main.py
echo app.include_router(notes.router)>> app\main.py
echo app.include_router(comments.router)>> app\main.py
echo.>> app\main.py
echo @app.get("/")>> app\main.py
echo def root():>> app\main.py
echo ^    return {"status": "ok", "message": "MindChat API attiva"}>> app\main.py

REM === database.py ===
echo import sqlite3> app\database.py
echo.>> app\database.py
echo def get_db():>> app\database.py
echo ^    conn = sqlite3.connect("mindchat.db")>> app\database.py
echo ^    conn.row_factory = sqlite3.Row>> app\database.py
echo ^    return conn>> app\database.py

REM === users.py ===
echo from fastapi import APIRouter> app\routers\users.py
echo from app.database import get_db>> app\routers\users.py
echo.>> app\routers\users.py
echo router = APIRouter(prefix="/users", tags=["users"])>> app\routers\users.py
echo.>> app\routers\users.py
echo @router.get("/{user_id}")>> app\routers\users.py
echo def get_user(user_id: int):>> app\routers\users.py
echo ^    db = get_db()>> app\routers\users.py
echo ^    cur = db.cursor()>> app\routers\users.py
echo ^    cur.execute("SELECT * FROM users WHERE id = ?", (user_id,))>> app\routers\users.py
echo ^    return cur.fetchone()>> app\routers\users.py

REM === notes.py ===
echo from fastapi import APIRouter> app\routers\notes.py
echo from app.database import get_db>> app\routers\notes.py
echo.>> app\routers\notes.py
echo router = APIRouter(prefix="/notes", tags=["notes"])>> app\routers\notes.py
echo.>> app\routers\notes.py
echo @router.get("/{user_id}")>> app\routers\notes.py
echo def get_notes(user_id: int):>> app\routers\notes.py
echo ^    db = get_db()>> app\routers\notes.py
echo ^    cur = db.cursor()>> app\routers\notes.py
echo ^    cur.execute("SELECT * FROM notes WHERE ownerId = ?", (user_id,))>> app\routers\notes.py
echo ^    return cur.fetchall()>> app\routers\notes.py

REM === comments.py ===
echo from fastapi import APIRouter> app\routers\comments.py
echo from app.database import get_db>> app\routers\comments.py
echo.>> app\routers\comments.py
echo router = APIRouter(prefix="/comments", tags=["comments"])>> app\routers\comments.py
echo.>> app\routers\comments.py
echo @router.get("/{note_id}")>> app\routers\comments.py
echo def get_comments(note_id: int):>> app\routers\comments.py
echo ^    db = get_db()>> app\routers\comments.py
echo ^    cur = db.cursor()>> app\routers\comments.py
echo ^    cur.execute(^">> app\routers\comments.py
echo ^        SELECT c.*, u.username>> app\routers\comments.py
echo ^        FROM comments c>> app\routers\comments.py
echo ^        JOIN users u ON u.id = c.userId>> app\routers\comments.py
echo ^        WHERE c.noteId = ?>> app\routers\comments.py
echo ^        ORDER BY c.timestamp ASC>> app\routers\comments.py
echo ^    ^", (note_id,))>> app\routers\comments.py
echo ^    return cur.fetchall()>> app\routers\comments.py

REM === requirements.txt ===
echo fastapi> requirements.txt
echo uvicorn>> requirements.txt

echo.
echo Backend creato con successo!
pause
