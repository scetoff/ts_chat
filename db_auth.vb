Public Class db_auth

    'Переменные для анимации окон
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    'Переменные для перемещения за любую часть формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Private Sub Stringer_MouseDown(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub Stringer_MouseMove(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub db_auth_MouseDown(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub db_auth_MouseMove(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Dim loggedin As Boolean = False
    Private Sub RadioButton2_CheckedChanged(sender As Object, e As EventArgs) Handles man_db.CheckedChanged
        If (man_db.Checked = True And edit.Checked = True) Then
            db_text.Enabled = True
        End If
    End Sub

    Private Sub RadioButton1_CheckedChanged(sender As Object, e As EventArgs) Handles auto_db.CheckedChanged
        If (auto_db.Checked = True) Then
            db_text.Enabled = False
        End If
    End Sub

    Private Sub login_btn_Click(sender As Object, e As EventArgs) Handles login_btn.Click

        'Проверка данных для подключения к серверу с БД
        My.Settings.Host = host_Text.Text
        My.Settings.User = user_Text.Text
        My.Settings.Pass = pass_Text.Text
        If (auto_db.Checked = True) Then
            My.Settings.auto_db = True
            My.Settings.man_db = True
        Else
            My.Settings.auto_db = False
            My.Settings.man_db = True
        End If
        My.Settings.dbName = db_text.Text
        My.Settings.Save()

        If (auto_db.Checked = True) Then
            Dim confirm = MsgBox("Вы выбрали авто-настройку БД, это означает, что приложение попытается создать новую базу данных с именем 'db_TSchat' на предоставленном хосте. Он может создать базу данных только если пользователь, указанный выше, имеет достаточно привилегий. Используйте авто-настройку БД, если вы хотите подключиться к существующей базе данных." + vbNewLine + "Вы хотите продолжить?", MsgBoxStyle.YesNo, "Продолжить")

            If (confirm = vbYes) Then
                loggedin = True
                Close()
            End If
        Else
            loggedin = True
            Close()
        End If

    End Sub

    Private Sub db_auth_FormClosing(sender As Object, e As FormClosingEventArgs) Handles MyBase.FormClosing
        If (loggedin = False) Then
            Application.Exit()
            End
        End If

    End Sub

    Private Sub close_btn_Click(sender As Object, e As EventArgs) Handles close_btn.Click
        Application.Exit()
        End

    End Sub

    Private Sub edit_CheckedChanged(sender As Object, e As EventArgs) Handles edit.CheckedChanged
        If (edit.Checked = True) Then
            host_Text.Enabled = True
            port_Text.Enabled = True
            user_Text.Enabled = True
            pass_Text.Enabled = True
            auto_db.Enabled = True
            man_db.Enabled = True

            If (man_db.Checked = True) Then
                db_text.Enabled = True
            End If
        Else
            host_Text.Enabled = False
            port_Text.Enabled = False
            user_Text.Enabled = False
            pass_Text.Enabled = False
            auto_db.Enabled = False
            man_db.Enabled = False
            db_text.Enabled = False
        End If
    End Sub

    Private Sub dblogindetails_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)

        host_Text.Text = My.Settings.Host
        port_Text.Text = My.Settings.port
        user_Text.Text = My.Settings.User
        pass_Text.Text = My.Settings.Pass
        If (My.Settings.auto_db = True) Then
            auto_db.Checked = True
        Else
            man_db.Checked = True
        End If
        db_text.Text = My.Settings.dbName


    End Sub
End Class