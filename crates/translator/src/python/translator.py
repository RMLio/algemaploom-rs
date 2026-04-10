import sys
import ltranslator

if __name__ == '__main__':
    mapping = sys.stdin.read()
    translated = ltranslator.translate(mapping)
    print(translated)
