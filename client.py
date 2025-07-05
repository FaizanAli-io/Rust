import requests

BASE_URL = "http://127.0.0.1:8080/books"

data = [
    {"id": 1, "title": "1984", "author": "George Orwell"},
    {"id": 2, "title": "Harry Potter", "author": "J. K. Rowling"},
    {"id": 3, "title": "Twilight Saga", "author": "Stephanie Mayer"},
]


def POST(new_book):
    response = requests.post(BASE_URL, json=new_book)
    print(f"POST /books → status {response.status_code}")


def GET():
    response = requests.get(BASE_URL)

    if response.ok:
        books = response.json()
        print("GET /books →")
        [print(book) for book in books]
    else:
        print("GET /books → status", response.status_code)


POST(data[2])
POST(data[1])
POST(data[0])
GET()
