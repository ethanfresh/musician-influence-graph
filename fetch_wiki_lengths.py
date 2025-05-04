import pandas as pd
import requests
import time
from urllib.parse import quote

df = pd.read_csv("/Users/ethanfreshman/Desktop/DS210/final/query_cleaned.csv")

def batch(iterable, n=50):
    for i in range(0, len(iterable), n):
        yield iterable[i:i + n]

def fetch_page_lengths(titles):
    title_str = "|".join(quote(title) for title in titles)
    url = f"https://en.wikipedia.org/w/api.php?action=query&titles={title_str}&prop=info&format=json"
    
    try:
        response = requests.get(url)
        pages = response.json()["query"]["pages"]
        return {page["title"]: page.get("length", 0) for page in pages.values()}
    except Exception as e:
        print("Error fetching batch:", e)
        return {title: None for title in titles}

all_titles = df["artistLabel"].dropna().unique().tolist()
lengths = {}

for i, title_batch in enumerate(batch(all_titles, 50)):
    batch_lengths = fetch_page_lengths(title_batch)
    lengths.update(batch_lengths)
    print(f"Fetched batch {i + 1}/{(len(all_titles) // 50) + 1}")
    time.sleep(0.1)  # light rate limiting

df["length"] = df["artistLabel"].map(lengths)

# Save the result
df.to_csv("/Users/ethanfreshman/Desktop/DS210/final/wikidata_with_lengths.csv", index=False)
print("Saved to wikidata_with_lengths.csv")
