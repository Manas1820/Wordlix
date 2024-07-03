# Wordlix

Wordlix is a powerful and efficient *(not rn, but hopefully)* Wordle solver written in Rust.

### How I got the dataset ?

#### 1. Download the n-gram data from the Google Books Ngram Viewer

Go to the [Google Books Ngram Viewer](https://books.google.com/ngrams) or use the dataset_downloader.py script to download the n-gram data.

#### 2. Filter and clean the n-gram data

Use the scipt written below to filter and clean the n-gram data.The script will filter out the n-grams that contain non-alphabetic characters, have a length not equal to 5, and sumup the frequency of the n-grams that are repeated.

```bash
find dataset -type f -name '*.txt' -exec awk 'length($1) == 5 && NF > 1 {sums[$1] += $2} END {for (word in sums) print word, sums[word]}' {} + | sort > filtered_n_gram_data.txt
```

Just for the sake of testing, I tried these commands

To fetch the number of n-grams in the filtered data

```bash
awk 'END {print NR}' filtered_n_gram_data.txt
```

To fetch the top 10 n-grams in the filtered data

```bash
awk '{print $2, $1}' filtered_n_gram_data.txt | sort -rn | head -10
```

#### 3. Find all the wordle words

- La words that can be guessed and which can be the word of the day
- Ta words that can be guessed but are never selected as the word of the da

```bash
awk 'NR==FNR { freq[$1] = $2; next } { if ($1 in freq) { print $1, freq[$1] } else { print $1, "1" } }' dataset/filtered_n_gram_data.txt dataset/combined_ans_guess.txt > dataset/wordle_words_x_n_gram.txt
```

Make a combined file of guesses and answers

```bash
cat dataset/wordle/guesses.txt dataset/wordle/ans.txt | sort | uniq > dataset/wordle/combined_ans_guess.txt
```
