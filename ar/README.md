# Asset Resolution

This module defines the abstract interface for USD's asset resolution plugin, so that clients can author asset references in their USD files that make sense to their asset management systems.
It also provides a "fallback resolver" that is active when no site-level plugin has been provided; the fallback resolver provides basic search-path based resolution.
