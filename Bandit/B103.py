import os
import stat

os.chmod('/etc/passwd', 07)
os.chmod('/etc/passwd', 0664)
