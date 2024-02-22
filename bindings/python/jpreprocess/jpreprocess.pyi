from typing import TypedDict, Literal, overload

__version__: str
JPREPROCESS_VERSION: str


class NjdObject(TypedDict):
    string: str
    pos: str
    pos_group1: str
    pos_group2: str
    pos_group3: str
    ctype: str
    cform: str
    orig: str
    read: str
    pron: str
    acc: int
    mora_size: int
    chain_rule: str
    chain_flag: int


class JPreprocess:
    def __init__(self, dictionary: str, user_dictionary: str |
                 None = None) -> None: ...

    def run_frontend(self, text: str) -> list[NjdObject]:
        """
        Run text-processing frontend.

        Arguments:
        - text (str): Input japanese text.

        Returns:
        - list[NjdObject]: list of NJDNode(s).
        """
    def make_label(self, njd_features: list[NjdObject]) -> list[str]:
        """
        Make full-context label using NjdObject

        Arguments:
        - njd_features (list[NjdObject]): list of NJDNode(s).

        Returns:
        - list[str]: list of full-context labels.
        """
    def extract_fullcontext(self, text: str) -> list[str]:
        """
        Extract full-context label from the input text.

        Arguments:
        - text (str): Input japanese text.

        Returns:
        - list[str]: list of full-context labels.
        """

    @overload
    def g2p(self, text: str, kana: bool = False,
            join: Literal[True] = True) -> str: ...

    @overload
    def g2p(self, text: str, kana: bool = False, *,
            join: Literal[False]) -> list[str]: ...

    @overload
    def g2p(self, text: str, kana: bool,
            join: Literal[False]) -> list[str]:
        """
        Grapheme-to-phoneme (G2P) conversion.

        Arguments:
        - text (str): Input japanese text.
        - kana (bool): Whether to generate alphabetical phoneme (False) or kana (True).
        - join (bool): Whether to generate a list of phoneme or kana (False) or join the output by delimiter (True).

        Returns:
        - list[str] (when join = False): list of phoneme (when kana = False) or kana (when kana = True).
        - str (when join = True): list of phoneme joined with space (when kana = False) or kana joined with empty string (when kana = True).
        """


def build_dictionary(input: str, output: str, user: bool = False,
                     serializer: Literal["jpreprocess", "lindera"] = "jpreprocess") -> None:
    """
    Build dictionary binary file(s).

    Arguments:
    - input (str): Path to source directory (system dictionary) or file (user dictionary).
    - output (str): Path to destination directory (system dictionary) or file (user dictionary).
    - user (bool): Whether to build system dictionary (False) or user dictionary (True). Default to False.
    - serializer (str): The name of serializer to use ("lindera" or "jpreprocess"). Default to "jpreprocess"
    """
