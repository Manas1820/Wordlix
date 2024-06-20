## Wordl



### 1. Download the n-gram data from the Google Books Ngram Viewer

Go to the [Google Books Ngram Viewer](https://books.google.com/ngrams) or use the dataset_downloader.py script to download the n-gram data.

### 2. Filter and clean the n-gram data

Use the scipt written below to filter and clean the n-gram data.The script will filter out the n-grams that contain non-alphabetic characters, have a length not equal to 5, and sumup the frequency of the n-grams that are repeated.

```bash
find dataset -type f -name '*.txt' -exec awk 'length($1) == 5 && NF > 1 {sums[$1] += $2} END {for (word in sums) print word, sums[word]}' {} + | sort > filtered_n_gram_data.txt
```
