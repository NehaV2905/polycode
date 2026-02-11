
import tree_sitter_languages
import tree_sitter

languages = ["python", "go", "java", "c", "rust", "ruby"]

print("Checking languages...")
for lang in languages:
    try:
        l = tree_sitter_languages.get_language(lang)
        print(f"PASS: {lang}")
    except Exception as e:
        print(f"FAIL: {lang} - {e}")
