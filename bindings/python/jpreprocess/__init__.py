import os
from .jpreprocess import *
from .dictionary import download_dictionary, dictionary_path


def jpreprocess(dictionary_version: str = f"v{JPREPROCESS_VERSION}", user_dictionary: str | None = None) -> JPreprocess:
    """
    Create jpreprocess instance with naist-jdic dictionary.

    If the system dictionary is not present, this function will download it.

    Arguments:
    - dictionary_version (str): Version of dictionary to download.
      We don't recommend specifying this argument unless you are aware of what you are doing.
    - user_dictionary (str | None): Path to user dictionary. The extionsion must be ".csv" or ".bin".
    """
    dict_path = dictionary_path(dictionary_version)
    if not os.path.exists(dict_path):
        download_dictionary(dictionary_version)
    return JPreprocess(dict_path, user_dictionary)
