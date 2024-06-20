import requests
import gzip
import re

class Downloader:
    @staticmethod
    def download_full_txt(language, n, alphabet):
        try:
            url = f"http://storage.googleapis.com/books/ngrams/books/googlebooks-{language}-all-{n}gram-20120701-{alphabet}.gz"
            pattern = re.compile(r'([a-zA-Z_]+)\t(\d{4})\t(\d+)\t(\d+)')
            file_name = f'datasets/{alphabet}.txt'

            # Fetch and decompress the file
            response = requests.get(url)
            response.raise_for_status()  # Check if the request was successful
            content = gzip.decompress(response.content).decode("utf-8")
            word_list = content.split("\n")

            with open(file_name, "w") as f_out:
                last_clean_word = ""
                last_clean_word_count = 0

                for line in word_list:
                    matcher = pattern.match(line)
                    if matcher:
                        word = matcher.group(1).lower().split("_")[0]

                        if word != last_clean_word:
                            # Avoid writing an empty line at the start
                            if last_clean_word and last_clean_word and last_clean_word_count:
                                f_out.write(f"{last_clean_word} {last_clean_word_count}\n")
                            last_clean_word = word
                            last_clean_word_count = int(matcher.group(3))
                        else:
                            last_clean_word_count += int(matcher.group(3))

                # Write the last word and its count
                if last_clean_word:
                    f_out.write(f"{last_clean_word} {last_clean_word_count}\n")

        except requests.RequestException as e:
            print(f"Error while downloading file: {e}")
        except Exception as e:
            print(f"An error occurred: {e}")

if __name__ == "__main__":
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    for letter in alphabet:
        Downloader.download_full_txt("eng", "1", letter)
        print(f"Downloaded {letter}.txt")
