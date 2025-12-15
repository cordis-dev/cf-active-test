import os
import stat

os.chmod('/etc/passwd', 0664)
os.chmod('/etc/passwd', 0o760)
