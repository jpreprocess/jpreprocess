from .jpreprocess import JPREPROCESS_VERSION
from contextlib import ExitStack
import sys
import os
import atexit

# from https://github.com/r9y9/pyopenjtalk/pull/74

if sys.version_info >= (3, 9):
    from importlib.resources import as_file, files
else:
    from importlib_resources import as_file, files

_file_manager = ExitStack()
atexit.register(_file_manager.close)
_file_ref = files(__package__)

# Dictionary path
# defaults to the package directory where the dictionary will be automatically downloaded
JPREPROCESS_DICT_PATH = os.environ.get(
    "JPREPROCESS_DICT_PATH",
    str(_file_manager.enter_context(
        as_file(_file_ref / JPREPROCESS_VERSION / "naist-jdic"))),
)
JPREPROCESS_DICT_URL = f"https://github.com/jpreprocess/jpreprocess/releases/download/v{JPREPROCESS_VERSION}/naist-jdic-jpreprocess.tar.gz"


def download_dictionary() -> None:
    from urllib.request import urlopen
    import tarfile
    import tempfile

    with tempfile.TemporaryFile() as file:
        print('Downloading: "{}"'.format(JPREPROCESS_DICT_URL))
        with urlopen(JPREPROCESS_DICT_URL) as response:
            try:
                from tqdm.auto import tqdm
                with tqdm.wrapattr(file, "write", total=getattr(response, "length", None)) as tar:
                    for chunk in response:
                        tar.write(chunk)
            except ImportError:
                for chunk in response:
                    file.write(chunk)
        file.seek(0)
        print("Extracting tar file")
        with tarfile.open(mode="r|gz", fileobj=file) as f, as_file(_file_ref / JPREPROCESS_VERSION) as dir:
            f.extractall(path=dir)
        print("done")
