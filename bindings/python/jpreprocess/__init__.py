import os
from .jpreprocess import *
from .dictionary import download_dictionary, JPREPROCESS_DICT_PATH


def jpreprocess(user_dictionary: str | None = None) -> JPreprocess:
    """
    Create jpreprocess instance with naist-jdic dictionary.

    If the system dictionary is not present, this function will download it.

    Arguments:
    - user_dictionary (str | None): Path to user dictionary. The extionsion must be ".csv" or ".bin".
    """
    if not os.path.exists(JPREPROCESS_DICT_PATH):
        download_dictionary()
    return JPreprocess(JPREPROCESS_DICT_PATH, user_dictionary)
