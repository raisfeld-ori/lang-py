from interfaces import Selection, Encrypt, Decrypt
from PySide6.QtWidgets import QApplication, QWidget, QFileDialog, QMainWindow, QMessageBox
from PySide6.QtCore import Qt
from PySide6.QtGui import QMouseEvent, QFont
import qdarktheme
from sys import exit
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import os

class File_Selection(Selection.Ui_Form, QWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.setFixedSize(537, 323)

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select_file_btn.clicked.connect(self.select_file)

    def select_file(self):
        self.file_selected = QFileDialog.getOpenFileName(
            parent=self,
            dir=os.path.expanduser("~/Desktop"),
            caption="תבחר קובץ"
        )[0]
        if self.file_selected != '':
            self.new_window = encrypt_window(self.file_selected)
            self.new_window.show()
            self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()

class encrypt_window(Encrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.decrypt.clicked.connect(self.show_decrypt)
        self.encryption_method.addItem("CBC mode")
        self.encryption_method_selected = AES.MODE_CBC
        self.failed_encrypting.hide()
        self.encrypt.clicked.connect(self.AES_encryption)
        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_encryption(self):
        try:
            self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)
            self.cipher = AES.new(self.key, self.encryption_method_selected)

            with open(self.real_file, "rb") as data:
                self.data = self.cipher.encrypt(pad(data.read(), AES.block_size))
            with open(self.real_file, "wb") as file:
                file.write(self.cipher.iv)
                file.write(self.data)

            window = QMessageBox()
            window.setText("ההצפנה הצליחה!")
            window.setFont(QFont("Secular One", 12))
            window.show()
            window.exec()

        except Exception as error:
            print(error.with_traceback(None))
            self.failed_encrypting.show()

    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()
    def show_decrypt(self):
        self.file_decryption_window = decrypt_window(self.real_file)
        self.file_decryption_window.show()
        self.close()
    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()
class decrypt_window(Decrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.encrypt.clicked.connect(self.show_encrypt)

        self.encryption_method_selected = AES.MODE_CBC
        self.wrong.hide()
        self.decryption_method.addItem("CBC mode")
        self.decrypt.clicked.connect(self.AES_decrypt)


        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_decrypt(self):
        self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)

        with open(self.real_file, "rb") as data:
            self.iv = data.read(16)
            self.cipher = AES.new(self.key, self.encryption_method_selected, iv=self.iv)
            self.data = unpad(self.cipher.decrypt(data.read()), AES.block_size)
        with open(self.real_file, "wb") as file:
            file.write(self.data)

        window = QMessageBox()
        window.setText("הפיענוח הצליח!")
        window.setFont(QFont("Secular One", 12))
        window.show()
        window.exec()


    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()

    def show_encrypt(self):
        self.file_encryption_window = encrypt_window(self.real_file)
        self.file_encryption_window.show()
        self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()

from interfaces import Selection, Encrypt, Decrypt
from PySide6.QtWidgets import QApplication, QWidget, QFileDialog, QMainWindow, QMessageBox
from PySide6.QtCore import Qt
from PySide6.QtGui import QMouseEvent, QFont
import qdarktheme
from sys import exit
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import os

class File_Selection(Selection.Ui_Form, QWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.setFixedSize(537, 323)

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select_file_btn.clicked.connect(self.select_file)

    def select_file(self):
        self.file_selected = QFileDialog.getOpenFileName(
            parent=self,
            dir=os.path.expanduser("~/Desktop"),
            caption="תבחר קובץ"
        )[0]
        if self.file_selected != '':
            self.new_window = encrypt_window(self.file_selected)
            self.new_window.show()
            self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()

class encrypt_window(Encrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.decrypt.clicked.connect(self.show_decrypt)
        self.encryption_method.addItem("CBC mode")
        self.encryption_method_selected = AES.MODE_CBC
        self.failed_encrypting.hide()
        self.encrypt.clicked.connect(self.AES_encryption)
        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_encryption(self):
        try:
            self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)
            self.cipher = AES.new(self.key, self.encryption_method_selected)

            with open(self.real_file, "rb") as data:
                self.data = self.cipher.encrypt(pad(data.read(), AES.block_size))
            with open(self.real_file, "wb") as file:
                file.write(self.cipher.iv)
                file.write(self.data)

            window = QMessageBox()
            window.setText("ההצפנה הצליחה!")
            window.setFont(QFont("Secular One", 12))
            window.show()
            window.exec()

        except Exception as error:
            print(error.with_traceback(None))
            self.failed_encrypting.show()

    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()
    def show_decrypt(self):
        self.file_decryption_window = decrypt_window(self.real_file)
        self.file_decryption_window.show()
        self.close()
    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()
class decrypt_window(Decrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.encrypt.clicked.connect(self.show_encrypt)

        self.encryption_method_selected = AES.MODE_CBC
        self.wrong.hide()
        self.decryption_method.addItem("CBC mode")
        self.decrypt.clicked.connect(self.AES_decrypt)


        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_decrypt(self):
        self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)

        with open(self.real_file, "rb") as data:
            self.iv = data.read(16)
            self.cipher = AES.new(self.key, self.encryption_method_selected, iv=self.iv)
            self.data = unpad(self.cipher.decrypt(data.read()), AES.block_size)
        with open(self.real_file, "wb") as file:
            file.write(self.data)

        window = QMessageBox()
        window.setText("הפיענוח הצליח!")
        window.setFont(QFont("Secular One", 12))
        window.show()
        window.exec()


    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()

    def show_encrypt(self):
        self.file_encryption_window = encrypt_window(self.real_file)
        self.file_encryption_window.show()
        self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()


from interfaces import Selection, Encrypt, Decrypt
from PySide6.QtWidgets import QApplication, QWidget, QFileDialog, QMainWindow, QMessageBox
from PySide6.QtCore import Qt
from PySide6.QtGui import QMouseEvent, QFont
import qdarktheme
from sys import exit
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import os

class File_Selection(Selection.Ui_Form, QWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.setFixedSize(537, 323)

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select_file_btn.clicked.connect(self.select_file)

    def select_file(self):
        self.file_selected = QFileDialog.getOpenFileName(
            parent=self,
            dir=os.path.expanduser("~/Desktop"),
            caption="תבחר קובץ"
        )[0]
        if self.file_selected != '':
            self.new_window = encrypt_window(self.file_selected)
            self.new_window.show()
            self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()

class encrypt_window(Encrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.decrypt.clicked.connect(self.show_decrypt)
        self.encryption_method.addItem("CBC mode")
        self.encryption_method_selected = AES.MODE_CBC
        self.failed_encrypting.hide()
        self.encrypt.clicked.connect(self.AES_encryption)
        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_encryption(self):
        try:
            self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)
            self.cipher = AES.new(self.key, self.encryption_method_selected)

            with open(self.real_file, "rb") as data:
                self.data = self.cipher.encrypt(pad(data.read(), AES.block_size))
            with open(self.real_file, "wb") as file:
                file.write(self.cipher.iv)
                file.write(self.data)

            window = QMessageBox()
            window.setText("ההצפנה הצליחה!")
            window.setFont(QFont("Secular One", 12))
            window.show()
            window.exec()

        except Exception as error:
            print(error.with_traceback(None))
            self.failed_encrypting.show()

    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()
    def show_decrypt(self):
        self.file_decryption_window = decrypt_window(self.real_file)
        self.file_decryption_window.show()
        self.close()
    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()
class decrypt_window(Decrypt.Ui_MainWindow, QMainWindow):
    def __init__(self, file: str):
        super().__init__()
        self.setupUi(self)
        self.real_file = file

        self.setWindowFlag(Qt.FramelessWindowHint)
        self.exit.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        self.light_mode.clicked.connect(lambda: qdarktheme.setup_theme("light"))
        self.dark_mode.clicked.connect(lambda: qdarktheme.setup_theme("dark"))
        self.select.clicked.connect(self.show_selection)
        self.encrypt.clicked.connect(self.show_encrypt)

        self.encryption_method_selected = AES.MODE_CBC
        self.wrong.hide()
        self.decryption_method.addItem("CBC mode")
        self.decrypt.clicked.connect(self.AES_decrypt)


        if len(self.real_file.split('/')[-1]) > len(self.real_file.split('\\')[-1]):
            self.current_file.setText(self.current_file.text() + file.split('\\')[-1])
        else:
            self.current_file.setText(self.current_file.text() + file.split('/')[-1])

    def AES_decrypt(self):
        self.key = PBKDF2(self.password.text().encode(), self.second_password.text().encode(), dkLen=32)

        with open(self.real_file, "rb") as data:
            self.iv = data.read(16)
            self.cipher = AES.new(self.key, self.encryption_method_selected, iv=self.iv)
            self.data = unpad(self.cipher.decrypt(data.read()), AES.block_size)
        with open(self.real_file, "wb") as file:
            file.write(self.data)

        window = QMessageBox()
        window.setText("הפיענוח הצליח!")
        window.setFont(QFont("Secular One", 12))
        window.show()
        window.exec()


    def show_selection(self):
        self.file_selection_window = File_Selection()
        self.file_selection_window.show()
        self.close()

    def show_encrypt(self):
        self.file_encryption_window = encrypt_window(self.real_file)
        self.file_encryption_window.show()
        self.close()


    def mousePressEvent(self, event: QMouseEvent) -> None:
        self.mouse_pos = event.globalPosition().toPoint()
    def mouseMoveEvent(self, event: QMouseEvent) -> None:
        self.move(self.pos() + event.globalPosition().toPoint() - self.mouse_pos)
        self.mouse_pos = event.globalPosition().toPoint()
        event.accept()




try:
    if __name__ == '__main__':
        app = QApplication()
        qdarktheme.setup_theme("dark")
        entry = File_Selection()
        entry.show()
        exit(app.exec())
except Exception as error:
    print(error)