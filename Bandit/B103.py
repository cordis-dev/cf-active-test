import os
import stat

os.chmod('/etc/passwd', 0o760)
os.chmod('/tmp/oh_hai', 0x1ff)
