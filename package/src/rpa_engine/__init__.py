import ctypes
import importlib.resources as resources
import shutil
import tempfile

ENGINE_DLL_NAME = "engine.dll"

with resources.files("rpa_engine").joinpath(ENGINE_DLL_NAME).open("rb") as bin:
    with tempfile.NamedTemporaryFile(delete=False, suffix=".dll") as tmp:
        shutil.copyfileobj(bin, tmp)
        dll_path = tmp.name

# NOTE: Use WinDLL instead of windll as the latter could pollutes caches
dll = ctypes.WinDLL(dll_path)


def mmv(dx: int, dy: int) -> None:
    """
    Move the mouse by dx and dy pixels.
    """
    dll.mmv(dx, dy)
