"""
All models used in the Nik-Lang project are defined here
"""

from .generic import Error


__version__ = "v1.0.0-NikLang"


__annotations__ = {
    "version": __version__,
    "Error": "Class for handling wrong input exceptions"
}


__all__ = [
    "__version__",
    "Error"
]