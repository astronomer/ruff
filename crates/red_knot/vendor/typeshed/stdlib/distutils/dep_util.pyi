def newer(source: str, target: str) -> bool: ...
def newer_pairwise(sources: list[str], targets: list[str]) -> list[tuple[str, str]]: ...
def newer_group(sources: list[str], target: str, missing: str = "error") -> bool: ...