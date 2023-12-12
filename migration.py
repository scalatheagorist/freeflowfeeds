import sqlite3
import json
import os

conn = sqlite3.connect('rss_feeds.db')
cursor = conn.cursor()

cursor.execute('''
    CREATE TABLE IF NOT EXISTS rss_feeds (
        id INTEGER PRIMARY KEY,
        author TEXT,
        title TEXT,
        link TEXT,
        publisher TEXT,
        lang TEXT,
        updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
''')

folder_path = '/Users/ronnywels/Coding/rust/freeflowfeeds/data'

for filename in os.listdir(folder_path):
    if filename.endswith('.json'):
        with open(os.path.join(folder_path, filename), 'r') as file:
            json_data = json.load(file)

            cursor.execute('''
                INSERT INTO rss_feeds (author, title, link, publisher, lang)
                VALUES (?, ?, ?, ?, ?)
            ''', (
                json_data.get('author'),
                json_data.get('article', {}).get('title'),
                json_data.get('article', {}).get('link'),
                json_data.get('publisher'),
                json_data.get('lang')
            ))

conn.commit()
conn.close()
