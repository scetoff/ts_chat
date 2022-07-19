'''''За основу взят код с сайта/форума https://ds-release.ru'''''

Imports vbicq4 'Плагин из-за которого пеерстал работать чат
Public Class frm_icq
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    Public WithEvents ICQ As New vbicq4.vbicq4
    Delegate Sub ErrDelegate(ByVal d As String)
    Delegate Sub MsgDelegate(ByVal u As String, ByVal m As String)
    Delegate Sub OnConDelegate()

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Private Sub trei_Click(sender As Object, e As EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(sender As Object, e As EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Private Sub frm_icq_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_icq_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub Stringer_MouseDown(sender As Object, e As MouseEventArgs) Handles Stringer.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub Stringer_MouseMove(sender As Object, e As MouseEventArgs) Handles Stringer.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub Coonect_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles Coonect.Click
        ICQ.UIN = txt_UIN.Text
        ICQ.pass = txt_Pass.Text
        ICQ.connect()
        Timer1.Start()
    End Sub
    ' Зашли в icq
    Private Sub ICQ_onConnect() Handles ICQ.onConnect
        If InvokeRequired Then
            Me.BeginInvoke(New OnConDelegate(AddressOf ICQ_onConnect))
            Exit Sub
        End If
        lbl_status.Text = "Статус : В сети"
        Coonect.Enabled = False
        cbox_spisok.SelectedIndex = 0
        If IO.File.Exists(IO.Directory.GetCurrentDirectory & "/Auth.wav") Then
            My.Computer.Audio.Play(IO.Directory.GetCurrentDirectory & "/Auth.wav")
        End If
    End Sub
    ' Вышли из icq
    Private Sub ICQ_onDisconnect() Handles ICQ.onDisconnect
        If InvokeRequired Then
            Me.BeginInvoke(New OnConDelegate(AddressOf ICQ_onDisconnect))
            Exit Sub
        End If
        lbl_status.Text = "Статус : Отключен"
        Coonect.Enabled = True
    End Sub
    ' Ошибка подключения
    Private Sub ICQ_onError(ByVal Description As String) Handles ICQ.onError
        If InvokeRequired Then
            Me.BeginInvoke(New ErrDelegate(AddressOf ICQ_onError), Description)
            Exit Sub
        End If
        lbl_status.Text = "Статус : Ошибка - " & Description
        Coonect.Enabled = True
    End Sub
    ' Принятие сообщений
    Private Sub icq_onMessage(ByVal uin As String,
                              ByVal message As String) Handles ICQ.onMessage
        If InvokeRequired Then
            Me.BeginInvoke(New MsgDelegate(AddressOf icq_onMessage),
                           New Object() {uin, message})
            Exit Sub
        End If
        If IO.File.Exists(IO.Directory.GetCurrentDirectory & "/PrinSms.wav") Then
            My.Computer.Audio.Play(IO.Directory.GetCurrentDirectory & "/PrinSms.wav")
        End If
        lbox_log.Items.Add("Сообщение от " & uin & " : " & message)
    End Sub
    ' Отправка сообщений
    Private Sub Send_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles Send.Click
        If ICQ.KeepAlive Then
            ICQ.SendMessage(UINSend.Text, TextSend.Text)
            lbox_log.Items.Add("Сообщение для " & UINSend.Text & " : " & TextSend.Text)
            If IO.File.Exists(IO.Directory.GetCurrentDirectory & "/SendSms.wav") Then
                My.Computer.Audio.Play(IO.Directory.GetCurrentDirectory & "/SendSms.wav")
            End If
            TextSend.Text = ""
        Else
            MsgBox("Выполните авторизацию", vbOKOnly, "Ошибка")
        End If
    End Sub
    ' Отключаемся
    Private Sub DisConnect_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles DisConnect.Click
        ICQ.DisConnect()
        Coonect.Enabled = True
        cbox_spisok.SelectedIndex = 12
    End Sub
    ' Следим за статусом
    Private Sub Timer1_Tick(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles Timer1.Tick
        lbl_status.Text = "Статус : " & ICQ.state
    End Sub
    Private Sub PassChar_CheckedChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles PassChar.CheckedChanged
        If PassChar.Checked = True Then
            txt_Pass.PasswordChar = ""
        Else
            txt_Pass.PasswordChar = "*"
        End If
    End Sub
    ' Выбор статуса
    Private Sub ComboBox1_SelectedIndexChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles cbox_spisok.SelectedIndexChanged
        Select Case cbox_spisok.SelectedIndex
            Case 0
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.online)
            Case 1
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.offline)
            Case 2
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.cruel)
            Case 3
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.away)
            Case 4
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.depr)
            Case 5
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.NA)
            Case 6
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.Occupied)
            Case 7
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.DND)
            Case 8
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.working)
            Case 9
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.home)
            Case 10
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.eat)
            Case 11
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.invisible)
            Case 12
                ICQ.SetStatus(vbicq4.vbicq4.ICQ_Status.unknow)
        End Select
    End Sub
    Private Sub TextSend_KeyDown(ByVal sender As Object, ByVal e As System.Windows.Forms.KeyEventArgs) Handles TextSend.KeyDown
        'Отправка сообщения нажатием на Enter
        If e.KeyCode = Keys.Enter Then
            Send.PerformClick()
        End If
    End Sub

    Private Sub frm_icq_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub

    Private Sub frm_icq_Load(sender As Object, e As EventArgs) Handles Me.Load
        Me.KeyPreview = True
    End Sub

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then

            lbl_status.BackColor = Color.White
            lbl_status.ForeColor = Color.Black
            lbl_uin.BackColor = Color.White
            lbl_uin.ForeColor = Color.Black
            lbl_pass.BackColor = Color.White
            lbl_pass.ForeColor = Color.Black
            PassChar.BackColor = Color.White
            PassChar.ForeColor = Color.Black
            lbl_who.BackColor = Color.White
            lbl_who.ForeColor = Color.Black
            Coonect.BackColor = Color.White
            Coonect.ForeColor = Color.Black
            DisConnect.BackColor = Color.White
            DisConnect.ForeColor = Color.Black
            Send.BackColor = Color.White
            Send.ForeColor = Color.Black
            lbox_log.BackColor = Color.White
            lbox_log.ForeColor = Color.Black
            GB_Auth.BackColor = Color.White
            GB_Message.BackColor = Color.White
            GB_LOG.BackColor = Color.White
            GB_Auth.ForeColor = Color.Black
            GB_Message.ForeColor = Color.Black
            GB_LOG.ForeColor = Color.Black
            Me.BackColor = Color.White

        Else

            'Тёмная тема
            lbl_status.BackColor = Color.Black
            lbl_status.ForeColor = Color.White
            lbl_uin.BackColor = Color.Black
            lbl_uin.ForeColor = Color.White
            lbl_pass.BackColor = Color.Black
            lbl_pass.ForeColor = Color.White
            PassChar.BackColor = Color.Black
            PassChar.ForeColor = Color.White
            lbl_who.BackColor = Color.Black
            lbl_who.ForeColor = Color.White
            Coonect.BackColor = Color.Black
            Coonect.ForeColor = Color.White
            DisConnect.BackColor = Color.Black
            DisConnect.ForeColor = Color.White
            Send.BackColor = Color.Black
            Send.ForeColor = Color.White
            lbox_log.BackColor = Color.Black
            lbox_log.ForeColor = Color.White
            GB_Auth.BackColor = Color.Black
            GB_Message.BackColor = Color.Black
            GB_LOG.BackColor = Color.Black
            GB_Auth.ForeColor = Color.White
            GB_Message.ForeColor = Color.White
            GB_LOG.ForeColor = Color.White
            Me.BackColor = Color.Black

        End If
    End Sub
End Class