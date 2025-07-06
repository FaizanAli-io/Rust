import requests

BASE_URL = "http://127.0.0.1:8080/books"

data = [
    {"title": "1984", "author": "George Orwell", "published": True},
    {"title": "Harry Potter", "author": "J. K. Rowling", "published": True},
    {"title": "Twilight Saga", "author": "Stephanie Mayer", "published": True},
]


def POST(new_book):
    response = requests.post(BASE_URL, json=new_book)
    print(f"POST /books → {response.status_code}")


def GET():
    response = requests.get(BASE_URL)

    if response.ok:
        print("GET /books →")
        [print(book) for book in response.json()]
    else:
        print("GET /books → status", response.status_code)


# POST(data[0])
# POST(data[1])
# POST(data[2])
GET()
