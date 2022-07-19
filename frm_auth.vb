Imports MySql.Data.MySqlClient
Imports System.Security.Cryptography
Imports System.Text.RegularExpressions
Public Class frm_Auth
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


    Private Sub frm_Auth_FormClosing(ByVal sender As Object, ByVal e As System.Windows.Forms.FormClosingEventArgs) Handles Me.FormClosing
        connection.Close()
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

    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub


    Dim action_signup As Boolean = False

    Private Sub main_formclosing() Handles MyBase.FormClosing

        connection.Close()
    End Sub
    Private Sub main_Load() Handles MyBase.Load

        'Загрузка данных о пользователе из системы
        set_con_string(db_auth.host_Text.Text, db_auth.port_Text.Text, db_auth.user_Text.Text, db_auth.pass_Text.Text, db_auth.db_text.Text)
        openconnection()
        InitilizeDatabase()


        If Not My.Settings.Username = String.Empty Then
            user.Text = My.Settings.Username

        End If

    End Sub


    Private Sub user_gotfocus() Handles user.GotFocus
        If user.Text = "Никнейм" Then

            user.Text = String.Empty
            lblstatus.Visible = False
        End If
    End Sub

    Private Sub user_lostfocus() Handles user.LostFocus
        If user.Text = String.Empty Then

            user.Text = "Никнейм"
        End If
    End Sub

    Private Sub pass_gotfocus() Handles pass.GotFocus
        If pass.Text = "Пароль" Then

            pass.Text = String.Empty
            pass.PasswordChar = "•"
            lblstatus.Visible = False
        End If
    End Sub

    Private Sub pass_lostfocus() Handles pass.LostFocus
        If pass.Text = String.Empty Then

            pass.Text = "Пароль"
            pass.PasswordChar = CChar(String.Empty)
        End If
    End Sub

    Private Sub email_gotfocus() Handles email.GotFocus
        If email.Text = "Почта" Then

            email.Text = String.Empty
            lblstatus.Visible = False
        End If
    End Sub

    Private Sub email_lostfocus() Handles email.LostFocus
        If email.Text = String.Empty Then

            email.Text = "Почта"
        End If
    End Sub

    Private Sub frm_auth__Load(sender As Object, e As EventArgs) Handles MyBase.Load

        If Checker.Checked = True Then
            switch_day.Visible = False
        Else
            switch_night.Visible = False
        End If

        Me.KeyPreview = True

        'Загрузка авторизации
        tav_img.Location = New Point(273, 95)
        GB_Register.Visible = False
        GB_Login.Visible = True

        logo.Location = New Point(345, 32)
        lblstatus.Parent = GB_Login
        lblstatus.Location = New Point(62, 308)
        PB_ICQ.Location = New Point(33, 196)

    End Sub
    Private Sub lblacnt_Click() Handles lbl_acnt.Click

        'Переключение между авторизацией и регистрацией

        If lbl_acnt.Text = "Создать аккаунт" Then
            lbl_acnt.Text = "Авторизация"
            lbl_acnt.Parent = GB_Register
            lbl_acnt.Location = New Point(71, 224)
            PB_ICQ.Visible = False
            action_signup = True

            tav_img.Location = New Point(4, 95)
            GB_Register.Visible = True
            GB_Login.Visible = False

            user.Parent = GB_Register
            user.Location = New Point(23, 58)
            pass.Parent = GB_Register
            pass.Location = New Point(23, 118)
            btnSignin.Parent = GB_Register
            btnSignin.Location = New Point(79, 224)
            btnSignin.Text = "Регистрация"
            lblstatus.Parent = GB_Register
            lblstatus.Location = New Point(50, 274)
            lblstatus.Text = ""
            lbl_acnt.Parent = GB_Register
            lbl_acnt.Location = New Point(63, 309)
            logo.Location = New Point(80, 32)

        ElseIf lbl_acnt.Text = "Авторизация" Then
            lbl_acnt.Text = "Создать аккаунт"
            lbl_acnt.Parent = GB_Login
            lbl_acnt.Location = New Point(62, 239)
            PB_ICQ.Visible = True
            action_signup = False


            lblstatus.Visible = False
            tav_img.Location = New Point(273, 95)
            GB_Register.Visible = False
            GB_Login.Visible = True

            user.Parent = GB_Login
            user.Location = New Point(33, 72)
            pass.Parent = GB_Login
            pass.Location = New Point(33, 145)
            btnSignin.Parent = GB_Login
            btnSignin.Location = New Point(89, 196)
            btnSignin.Text = "Вход"
            lblstatus.Parent = GB_Login
            lblstatus.Location = New Point(62, 308)
            lblstatus.Text = ""
            lbl_acnt.Parent = GB_Login
            lbl_acnt.Location = New Point(62, 337)
            logo.Location = New Point(345, 32)
        End If
    End Sub

    Private Sub btnSignin_click() Handles btnSignin.Click

        'Проверка авторизации
        If user.Text = "Никнейм" Or user.Text = String.Empty Then
            lblstatus.Visible = True
            lblstatus.Text = "Неверный никнейм"
            lblstatus.ForeColor = Color.DarkRed
            Exit Sub
        End If

        If pass.Text = "Пароль" Or pass.Text = String.Empty Then
            lblstatus.Visible = True
            lblstatus.Text = "Неверный пароль"
            lblstatus.ForeColor = Color.DarkRed
            Exit Sub
        End If

        Dim insertcommand0 As New MySqlCommand("UPDATE tb_chat SET chat = 'Добро пожаловать в Таверну Странника'", connection)
        insertcommand0.ExecuteNonQuery()
        If action_signup = False Then
            Dim usrfound As Boolean = False
            Dim readcommand As New MySqlCommand("SELECT * FROM tb_users WHERE username='" & user.Text & "'", connection)
            Dim datareader As MySqlDataReader

            Try
                datareader = readcommand.ExecuteReader
                While datareader.Read()
                    If datareader(1).ToString = user.Text And datareader(2).ToString = md5encrypter(pass.Text) Then
                        My.Settings.Username = datareader(1).ToString
                        My.Settings.Email = datareader(3).ToString
                        My.Settings.FirstName = datareader(4).ToString
                        My.Settings.Picture = datareader(5).ToString
                        My.Settings.telPhone = datareader(6).ToString
                        My.Settings.ChatColor = datareader(7).ToString
                        My.Settings.ChatFont = datareader(8).ToString
                        My.Settings.ChatFontSize = CInt(datareader(9).ToString)
                        My.Settings.ChatFontStyle = datareader(10).ToString
                        My.Settings.Save()
                        usrfound = True
                        Exit While
                    End If
                End While

                datareader.Close()
                If usrfound = True Then
                    lblstatus.Text = ""
                    Me.Hide()
                    Dim chatwin As New frm_chat
                    chatwin.ShowDialog()
                Else
                    lblstatus.Visible = True
                    lblstatus.ForeColor = Color.DarkRed
                    lblstatus.Text = "Проверьте данные"
                End If
            Catch
                lblstatus.Visible = True
                lblstatus.ForeColor = Color.DarkRed
                lblstatus.Text = "Ошибка чтения БД"
            End Try

        End If

        'Проверка регистрации
        If action_signup = True Then
            If email.Text = "Почта" Or email.Text = String.Empty Then
                lblstatus.Visible = True
                lblstatus.Text = "Неверная почта"
                lblstatus.ForeColor = Color.DarkRed
                Exit Sub
            End If

            Dim readcommand As New MySqlCommand("SELECT * FROM tb_users", connection)
            Dim datareader As MySqlDataReader
            Try
                datareader = readcommand.ExecuteReader()
                While datareader.Read()

                    If datareader(1).ToString = user.Text Then
                        lblstatus.Visible = True
                        lblstatus.Text = "Никнейм уже занят"
                        lblstatus.ForeColor = Color.DarkRed
                        datareader.Close()
                        Exit Sub
                    End If
                    If datareader(3).ToString = email.Text Then
                        lblstatus.Visible = True
                        lblstatus.Text = "Почта уже занята"
                        lblstatus.ForeColor = Color.DarkRed
                        datareader.Close()
                        Exit Sub
                    End If
                End While
                datareader.Close()

            Catch
                lblstatus.Visible = True
                lblstatus.ForeColor = Color.DarkRed
                lblstatus.Text = "Ошибка чтения БД"
            End Try

            If Not IO.File.Exists(Application.StartupPath & "\users.png") Then
                MsgBox("Файл с именем users.png не найден. Пожалуйста переустановите програму.", MsgBoxStyle.Critical, "IO Error")
                Exit Sub
            End If

            Dim insertcommand As New MySqlCommand("INSERT INTO tb_users (username, password, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle) VALUES('" & user.Text & "','" & md5encrypter(pass.Text) & "','" & email.Text & "','" & user.Text & "','" & imgtobytes(Application.StartupPath & "\users.png") & "','+7(xxx)xxx-xx-xx', 'FF000000', 'Candara', '16', 'Regular')", connection)
            Try
                insertcommand.ExecuteNonQuery()
                lblstatus.Visible = True
                lblstatus.Text = "Аккаунт создан"
                lblstatus.ForeColor = Color.DarkGreen
            Catch
                lblstatus.Visible = True
                lblstatus.ForeColor = Color.DarkRed
                lblstatus.Text = "Ошибка чтения БД"
            End Try
        End If
    End Sub

    Private Sub PB_ICQ_Click(sender As Object, e As EventArgs) Handles PB_ICQ.Click

        'Уведомление касательно ICQ
        MsgBox("Некоторое время назад была прекращена поддержка плагина для icq, причиной, послужившей этому могли стать брут-программы, но данная информация является не подтверждённой", vbOKOnly, "Предупреждение")
        If frm_icq.Visible = True Then
            MsgBox("Аська уже запущена", vbOKOnly, "Предупреждение")
        Else
            frm_icq.Show()
        End If
    End Sub

    'Вход по нажатию на Enter
    Private Sub pass_KeyDown(sender As Object, e As KeyEventArgs) Handles pass.KeyDown
        If e.KeyCode = Keys.Enter Then
            pass.AppendText("")
            e.SuppressKeyPress = True
        End If

        If e.KeyCode = Keys.Enter Then
            e.Handled = True
            Call btnSignin_click()
        Else
            e.Handled = False
        End If
    End Sub
    Private Sub user_KeyDown(sender As Object, e As KeyEventArgs) Handles user.KeyDown
        If e.KeyCode = Keys.Enter Then
            user.AppendText("")
            e.SuppressKeyPress = True
        End If

        If e.KeyCode = Keys.Enter Then
            e.Handled = True
            Call btnSignin_click()
        Else
            e.Handled = False
        End If
    End Sub

    Private Sub frm_Auth_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub

    Private Sub Switch_night_Click(sender As Object, e As EventArgs) Handles switch_night.Click
        switch_night.Visible = False
        switch_day.Visible = True
        Checker.Checked = False
        My.Settings.Checking = Checker.Checked
        My.MySettings.Default.Save()


        'Тёмная тема
        'Форма авторизации

        lbl_Login.BackColor = Color.Black
        lbl_Login.ForeColor = Color.White
        lbl_Pass.BackColor = Color.Black
        lbl_Pass.ForeColor = Color.White
        lbl_Name.BackColor = Color.Black
        lbl_Name.ForeColor = Color.White
        lbl_PassCurrent.BackColor = Color.Black
        lbl_PassCurrent.ForeColor = Color.White
        lbl_Mail.BackColor = Color.Black
        lbl_Mail.ForeColor = Color.White
        lbl_acnt.BackColor = Color.Black
        lbl_acnt.ForeColor = Color.White

        GB_Login.ForeColor = Color.White
        GB_Register.ForeColor = Color.White
        GB_Login.BackColor = Color.Black
        GB_Register.BackColor = Color.Black
        Me.BackColor = Color.Black
        btnSignin.BackColor = Color.Black
        btnSignin.ForeColor = Color.White
        btnSignin.FlatAppearance.BorderColor = Color.White

    End Sub

    Private Sub Switch_day_Click(sender As Object, e As EventArgs) Handles switch_day.Click
        switch_night.Visible = True
        switch_day.Visible = False
        Checker.Checked = True
        My.Settings.Checking = Checker.Checked
        My.MySettings.Default.Save()

        'Светлая тема
        'Форма аторизации

        lbl_Login.BackColor = Color.White
        lbl_Login.ForeColor = Color.Black
        lbl_Pass.BackColor = Color.White
        lbl_Pass.ForeColor = Color.Black
        lbl_Name.BackColor = Color.White
        lbl_Name.ForeColor = Color.Black
        lbl_PassCurrent.BackColor = Color.White
        lbl_PassCurrent.ForeColor = Color.Black
        lbl_Mail.BackColor = Color.White
        lbl_Mail.ForeColor = Color.Black
        lbl_acnt.BackColor = Color.White
        lbl_acnt.ForeColor = Color.Black

        GB_Login.ForeColor = Color.Black
        GB_Register.ForeColor = Color.Black
        GB_Login.BackColor = Color.White
        GB_Register.BackColor = Color.White
        Me.BackColor = Color.White
        btnSignin.BackColor = Color.White
        btnSignin.ForeColor = Color.Black
        btnSignin.FlatAppearance.BorderColor = Color.Black

    End Sub

    Private Sub frm_Auth_FormClosed(sender As Object, e As FormClosedEventArgs) Handles Me.FormClosed
        My.MySettings.Default.Checking = Checker.Checked
        My.MySettings.Default.Save()
    End Sub

    Public Sub New()
        InitializeComponent()
        Checker.Checked = My.Settings.Checking

        If Checker.Checked = True Then
            Switch_day_Click(switch_day, Nothing)
        Else
            Switch_night_Click(switch_day, Nothing)
        End If
    End Sub
End Class