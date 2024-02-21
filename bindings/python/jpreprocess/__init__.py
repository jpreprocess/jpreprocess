from .jpreprocess import *

__all__ = [
    "download_dictionary", "jpreprocess", "JPREPROCESS_DICT_PATH", "JPREPROCESS_DICT_URL",
    *jpreprocess.__all__,
]

import os
import pkg_resources

# Dictionary path
# defaults to the package directory where the dictionary will be automatically downloaded
JPREPROCESS_DICT_PATH = os.environ.get(
    "JPREPROCESS_DICT_PATH",
    pkg_resources.resource_filename(
        __name__, f"{JPREPROCESS_VERSION}/naist-jdic"),
)
JPREPROCESS_DICT_URL = f"https://github.com/jpreprocess/jpreprocess/releases/download/v{JPREPROCESS_VERSION}/naist-jdic-jpreprocess.tar.gz"

def download_dictionary():
    global JPREPROCESS_DICT_PATH

    from urllib import request as urllib
    import tarfile

    filename = pkg_resources.resource_filename(
        __name__, "naist-jdic-jpreprocess.tar.gz")
    print(f'Downloading: "{JPREPROCESS_DICT_URL}"')

    try:
        from tqdm.auto import tqdm

        class TqdmUpTo(tqdm):
            def __init__(self):
                super(TqdmUpTo, self).__init__(
                    unit="B",
                    unit_scale=True,
                    unit_divisor=1024,
                    miniters=1,
                    desc="naist-jdic-jpreprocess.tar.gz",
                )

            def update_to(self, b=1, bsize=1, tsize=None):
                if tsize is not None:
                    self.total = tsize
                return self.update(b * bsize - self.n)

        with TqdmUpTo() as t:
            urllib.urlretrieve(JPREPROCESS_DICT_URL, filename,
                               reporthook=t.update_to)
            t.total = t.n

    except ImportError:
        urllib.urlretrieve(JPREPROCESS_DICT_URL, filename)

    print(f"Extracting tar file {filename}")
    extract_dir = pkg_resources.resource_filename(
        __name__, JPREPROCESS_VERSION)
    os.mkdir(extract_dir)
    with tarfile.open(filename, mode="r|gz") as f:
        f.extractall(path=extract_dir)
    os.remove(filename)


def jpreprocess(user_dictionary: str | None = None):
    if not os.path.exists(JPREPROCESS_DICT_PATH):
        download_dictionary()
    return JPreprocess(JPREPROCESS_DICT_PATH, user_dictionary)
