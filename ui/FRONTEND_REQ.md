# Frontend Requirements

## Overview

The frontend source code is located inside the `ui/` folder.

---

## Requirements

### Install Node.js (Includes npm)

Download and install Node.js from:

https://nodejs.org/

Recommended version:
- Node.js >= 18.x

Verify installation:

```bash
node -v
npm -v
```

---

## Setup & Running the Frontend

### 1. Clone the Repository

```bash
git clone <repository-url>
cd <project-folder>
```

### 2. Navigate to the `ui` Folder and Install Dependencies

```bash
cd ui
npm install
```

### 3. Run or Build the Project

To start the development server:

```bash
npm run dev
```

The app will run at:

```
http://localhost:5173/
```

OR to create a production build:

```bash
npm run build
```

The optimized files will be generated inside:

```
ui/dist/
```