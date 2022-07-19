Imports MySql.Data.MySqlClient
Public Class frm_FriendsProfile
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

            lbl_frnd.BackColor = Color.White
            lbl_frnd.ForeColor = Color.Black
            pnl_frnd.BackColor = Color.White
            pnl_frnd.ForeColor = Color.Black
            lbl_NameS.BackColor = Color.White
            lbl_NameS.ForeColor = Color.Black
            lbl_Name.BackColor = Color.White
            lbl_Name.ForeColor = Color.Black
            lbl_Nickname.BackColor = Color.White
            lbl_Nickname.ForeColor = Color.Black
            lbl_mail.BackColor = Color.White
            lbl_mail.ForeColor = Color.Black
            lbl_phone.BackColor = Color.White
            lbl_phone.ForeColor = Color.Black
            lbl_MailS.BackColor = Color.White
            lbl_MailS.ForeColor = Color.Black
            lbl_NicknameS.BackColor = Color.White
            lbl_NicknameS.ForeColor = Color.Black
            lbl_PhoneS.BackColor = Color.White
            lbl_PhoneS.ForeColor = Color.Black
            lbl_Send.BackColor = Color.White
            lbl_Send.ForeColor = Color.Black
            GB_Frnd.BackColor = Color.White
            GB_Info.BackColor = Color.White
            Me.BackColor = Color.White
            Friend_Picture.BackColor = Color.White

            sendmes_pic.Image = My.Resources.Send_mail_b
        Else

            'Тёмная тема
            lbl_frnd.BackColor = Color.Black
            lbl_frnd.ForeColor = Color.White
            pnl_frnd.BackColor = Color.Black
            pnl_frnd.ForeColor = Color.White
            lbl_NameS.BackColor = Color.Black
            lbl_NameS.ForeColor = Color.White
            lbl_Name.BackColor = Color.Black
            lbl_Name.ForeColor = Color.White
            lbl_Nickname.BackColor = Color.Black
            lbl_Nickname.ForeColor = Color.White
            lbl_mail.BackColor = Color.Black
            lbl_mail.ForeColor = Color.White
            lbl_phone.BackColor = Color.Black
            lbl_phone.ForeColor = Color.White
            lbl_PhoneS.BackColor = Color.Black
            lbl_PhoneS.ForeColor = Color.White
            lbl_NicknameS.BackColor = Color.Black
            lbl_NicknameS.ForeColor = Color.White
            lbl_PhoneS.BackColor = Color.Black
            lbl_PhoneS.ForeColor = Color.White
            lbl_Send.BackColor = Color.Black
            lbl_Send.ForeColor = Color.White
            GB_Frnd.BackColor = Color.Black
            GB_Info.BackColor = Color.Black
            Me.BackColor = Color.Black
            Friend_Picture.BackColor = Color.Black

            sendmes_pic.Image = My.Resources.Send_mail
        End If
    End Sub

    Private Sub frm_FriendsProfile_Load(ByVal sender As Object, ByVal e As System.EventArgs) Handles Me.Load
        Me.KeyPreview = True
        'AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)

        tempconopen()

        Dim readcommand As New MySqlCommand("SELECT * FROM tb_users WHERE username='" & frndusrname & "'", tempcon)
        Dim datareader As MySqlDataReader
        datareader = readcommand.ExecuteReader

        While datareader.Read()
            lbl_NameS.Text = datareader(1).ToString()
            lbl_MailS.Text = datareader(3).ToString()
            lbl_NicknameS.Text = datareader(4).ToString()
            lbl_PhoneS.Text = datareader(6).ToString()
            Me.Text = Me.Text & " - " & lbl_NicknameS.Text
            Dim mems As System.IO.MemoryStream = New System.IO.MemoryStream(Convert.FromBase64String(datareader(5).ToString()))
            Friend_Picture.Image = Drawing.Image.FromStream(mems)
        End While
        datareader.Close()
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

    Private Sub PictureBox2_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles sendmes_pic.Click
        Dim PrivateMessageForm As New frm_PrivateMessage
        frm_PrivateMessage.ShowDialog()
    End Sub

    Private Sub frm_FriendsProfile_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class