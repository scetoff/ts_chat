Imports MySql.Data.MySqlClient
Public Class frm_EditProfile
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then
            pnl_red.BackColor = Color.White
            pnl_red.ForeColor = Color.Black
            lbl_redakt.BackColor = Color.White
            lbl_redakt.ForeColor = Color.Black
            lbl_Name.BackColor = Color.White
            lbl_Name.ForeColor = Color.Black
            lbl_Nickname.BackColor = Color.White
            lbl_Nickname.ForeColor = Color.Black
            lbl_mail.BackColor = Color.White
            lbl_mail.ForeColor = Color.Black
            lbl_phone.BackColor = Color.White
            lbl_phone.ForeColor = Color.Black
            ChangeBoxX.BackColor = Color.White
            ChangeBoxX.ForeColor = Color.Black
            lbl_Pass.BackColor = Color.White
            lbl_Pass.ForeColor = Color.Black
            lbl_repeatpass.BackColor = Color.White
            lbl_repeatpass.ForeColor = Color.Black
            btn_save.BackColor = Color.White
            btn_save.ForeColor = Color.Black
            mask_phone.BackColor = Color.White
            mask_phone.ForeColor = Color.Black
            GB_Pass.BackColor = Color.White
            GB_redakt.BackColor = Color.White
            Me.BackColor = Color.White

        Else

            'Тёмная тема
            pnl_red.BackColor = Color.Black
            pnl_red.ForeColor = Color.White
            lbl_redakt.BackColor = Color.Black
            lbl_redakt.ForeColor = Color.White
            lbl_Name.BackColor = Color.Black
            lbl_Name.ForeColor = Color.White
            lbl_Nickname.BackColor = Color.Black
            lbl_Nickname.ForeColor = Color.White
            lbl_mail.BackColor = Color.Black
            lbl_mail.ForeColor = Color.White
            lbl_phone.BackColor = Color.Black
            lbl_phone.ForeColor = Color.White
            ChangeBoxX.BackColor = Color.Black
            ChangeBoxX.ForeColor = Color.White
            lbl_Pass.BackColor = Color.Black
            lbl_Pass.ForeColor = Color.White
            lbl_repeatpass.BackColor = Color.Black
            lbl_repeatpass.ForeColor = Color.White
            btn_save.BackColor = Color.Black
            btn_save.ForeColor = Color.White
            mask_phone.BackColor = Color.Black
            mask_phone.ForeColor = Color.White
            GB_Pass.BackColor = Color.Black
            GB_redakt.BackColor = Color.Black
            Me.BackColor = Color.Black
        End If

    End Sub
    Private Sub frm_EditProfile_FormClosing(ByVal sender As Object, ByVal e As System.Windows.Forms.FormClosingEventArgs) Handles Me.FormClosing
        tempcon.Close()
    End Sub
    Private Sub frm_auth_MouseDown(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_auth_MouseMove(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub


    Private Sub Stringer_MouseDown(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub Stringer_MouseMove(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseMove
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

    Private Sub ChangeBoxX_CheckedChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ChangeBoxX.CheckedChanged
        'Разрешить вносить изменения
        If ChangeBoxX.Checked = True Then
            txt_pass.ReadOnly = False
            txt_repeatpass.ReadOnly = False
        Else
            txt_pass.ReadOnly = True
            txt_repeatpass.ReadOnly = True
        End If
    End Sub

    Private Sub frm_EditProfile_Load(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MyBase.Load

        Me.KeyPreview = True
        'AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)

        tempconopen()

        txt_Name.Text = My.Settings.FirstName
        txt_mail.Text = My.Settings.Email
        mask_phone.Text = My.Settings.telPhone
        txt_Nickname.Text = My.Settings.Username

    End Sub

    Private Sub btn_save_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles btn_save.Click
        'Проверка новых учётных данных
        Dim cmdread As New MySqlCommand("SELECT * FROM tb_users", tempcon)
        Dim datareader As MySqlDataReader
        Try
            datareader = cmdread.ExecuteReader
            While datareader.Read()
                If datareader(1).ToString = txt_Nickname.Text And Not My.Settings.Username = txt_Nickname.Text Then
                    MsgBox("Никнейм уже занят.", MsgBoxStyle.Exclamation, "Никнейм уже занят")
                    datareader.Close()
                    Exit Sub
                End If
                If datareader(3).ToString = txt_mail.Text And Not My.Settings.Email = txt_mail.Text Then
                    MsgBox("Почта уже занята.", MsgBoxStyle.Exclamation, "Почта уже занята")
                    datareader.Close()
                    Exit Sub
                End If
                If datareader(6).ToString = mask_phone.Text And Not My.Settings.telPhone = mask_phone.Text Then
                    MsgBox("Номер телефона уже занят.", MsgBoxStyle.Exclamation, "Телефон уже занят")
                    datareader.Close()
                    Exit Sub
                End If
            End While
            datareader.Close()
        Catch ex As Exception
            MsgBox("Ошибка", MsgBoxStyle.Critical, "Ошибка соединения с БД")
        End Try
        Try

            If ChangeBoxX.Checked = True Then
                If txt_pass.Text = txt_repeatpass.Text And Not txt_pass.Text = String.Empty Then
                    Dim updateusr As New MySqlCommand("UPDATE tb_users SET username='" & txt_Nickname.Text & "', email='" & txt_mail.Text & "', name='" & txt_Name.Text & "', mobile='" & mask_phone.Text & "', password='" & md5encrypter(txt_pass.Text) & "' WHERE username='" & My.Settings.Username & "'", tempcon)
                    updateusr.ExecuteNonQuery()
                Else
                    MsgBox("Пароль не существует или введён неверно.", MsgBoxStyle.Exclamation, "Ошибка ввода пароля")
                End If
            Else
                Dim updateusr As New MySqlCommand("UPDATE tb_users SET username='" & txt_Nickname.Text & "', email='" & txt_mail.Text & "', name='" & txt_Name.Text & "', mobile='" & mask_phone.Text & "' WHERE username='" & My.Settings.Username & "'", tempcon)
                updateusr.ExecuteNonQuery()
            End If
            My.Settings.FirstName = txt_Name.Text
            My.Settings.Username = txt_Nickname.Text
            My.Settings.Email = txt_mail.Text
            My.Settings.telPhone = mask_phone.Text
            My.Settings.Save()
            MsgBox("Профиль сохранён.", MsgBoxStyle.Information, "Изменения прошли успешно" & vbNewLine & "Сохраненные изменения появятся при следующем входе в систему!")
        Catch ex As Exception
            MsgBox("Ошибка", MsgBoxStyle.Critical, "Ошибка соединения с БД")
        End Try
    End Sub

    Private Sub frm_EditProfile_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class