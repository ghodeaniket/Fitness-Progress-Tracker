# Fitness Progress Tracker API Documentation

This document provides an overview of the API endpoints available in the Fitness Progress Tracker backend.

## Base URL

All API endpoints are prefixed with `/api/v1`.

## Authentication

Most endpoints require authentication using JWT (JSON Web Token). To authenticate, include the token in the Authorization header:

```
Authorization: Bearer <your_token>
```

## Endpoints

### Authentication

#### Register a new user

- **URL**: `/auth/register`
- **Method**: `POST`
- **Authentication**: Not required
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "username": "username",
    "password": "password123",
    "first_name": "John",
    "last_name": "Doe"
  }
  ```
- **Response**: `201 Created`
  ```json
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "username": "username",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2025-03-21T10:00:00Z"
  }
  ```

#### Login

- **URL**: `/auth/login`
- **Method**: `POST`
- **Authentication**: Not required
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "password": "password123"
  }
  ```
- **Response**: `200 OK`
  ```json
  {
    "user": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "email": "user@example.com",
      "username": "username",
      "first_name": "John",
      "last_name": "Doe",
      "created_at": "2025-03-21T10:00:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```

### User

#### Get User Profile

- **URL**: `/users/profile`
- **Method**: `GET`
- **Authentication**: Required
- **Response**: `200 OK`
  ```json
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "username": "username",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2025-03-21T10:00:00Z"
  }
  ```

### Workouts

#### Create a Workout

- **URL**: `/workouts`
- **Method**: `POST`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "name": "Morning Cardio",
    "description": "30 minute cardio session",
    "date": "2025-03-21T08:00:00Z",
    "duration": 1800,
    "calories_burned": 350,
    "exercises": [
      {
        "exercise_id": "123e4567-e89b-12d3-a456-426614174000",
        "sets": 3,
        "reps": 12,
        "weight": 20.5,
        "duration": 600,
        "distance": null,
        "notes": "Felt good, increased weight"
      }
    ]
  }
  ```
- **Response**: `201 Created`
  ```json
  {
    "id": "123e4567-e89b-12d3-a456-426614174000"
  }
  ```

#### Get All Workouts

- **URL**: `/workouts`
- **Method**: `GET`
- **Authentication**: Required
- **Response**: `200 OK`
  ```json
  [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "user_id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Morning Cardio",
      "description": "30 minute cardio session",
      "date": "2025-03-21T08:00:00Z",
      "duration": 1800,
      "calories_burned": 350,
      "created_at": "2025-03-21T10:00:00Z",
      "updated_at": "2025-03-21T10:00:00Z"
    }
  ]
  ```

#### Get Workout Details

- **URL**: `/workouts/{workout_id}`
- **Method**: `GET`
- **Authentication**: Required
- **Response**: `200 OK`
  ```json
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Morning Cardio",
    "description": "30 minute cardio session",
    "date": "2025-03-21T08:00:00Z",
    "duration": 1800,
    "calories_burned": 350,
    "exercises": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "exercise": {
          "id": "123e4567-e89b-12d3-a456-426614174000",
          "name": "Bench Press",
          "description": "Chest exercise",
          "category": "strength",
          "created_at": "2025-03-21T10:00:00Z",
          "updated_at": "2025-03-21T10:00:00Z"
        },
        "sets": 3,
        "reps": 12,
        "weight": 20.5,
        "duration": 600,
        "distance": null,
        "notes": "Felt good, increased weight"
      }
    ]
  }
  ```

#### Delete a Workout

- **URL**: `/workouts/{workout_id}`
- **Method**: `DELETE`
- **Authentication**: Required
- **Response**: `204 No Content`

## Error Responses

All endpoints may return the following error responses:

- **400 Bad Request**: Invalid request parameters
- **401 Unauthorized**: Missing or invalid authentication
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server error

Error responses are formatted as follows:

```json
{
  "error": "Error message"
}
```
