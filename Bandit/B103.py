import os
import stat

os.chmod('~/.bashrc', 511)
os.chmod('/etc/hosts', 0o777)
os.chmod('/tmp/oh_hai', 0x1ff)
os.chmod('/etc/passwd', stat.S_IRWXU)
