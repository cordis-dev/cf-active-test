import os
import stat

os.chmod('/etc/passwd', 0o777)
os.chmod('/etc/passwd', 0664)
