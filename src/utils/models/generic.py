"""
This module contains the basic models for the Nik Lang
"""

from src.utils.base.libraries import BaseModel, Field


class Error(BaseModel):
    """
    Error model
    """
    message: str = Field(..., title="Message", description="Error message")

    class Config:
        """
        Configuration for the model
        """
        schema_extra = {
            "example": {
                "message": "Error message"
            }
        }
