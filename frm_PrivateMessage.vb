Imports MySql.Data.MySqlClient
Imports System.Drawing.Drawing2D
Public Class frm_PrivateMessage

    Dim Index As Integer = 0
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub
    '''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
    Private Sub frm_PrivateMessage_Load(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MyBase.Load

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then

            lbl_lich.BackColor = Color.White
            lbl_lich.ForeColor = Color.Black
            pnl_lich.BackColor = Color.White
            lbl_Komu.BackColor = Color.White
            lbl_Komu.ForeColor = Color.Black
            lbl_text.BackColor = Color.White
            lbl_text.ForeColor = Color.Black
            GB_lich.BackColor = Color.White
            GB_lich.ForeColor = Color.Black
            sendmes.BackColor = Color.White
            sendmes.Image = My.Resources.Send_mail_b
            Me.BackColor = Color.White
        Else

            'Тёмная тема
            lbl_lich.BackColor = Color.Black
            lbl_lich.ForeColor = Color.White
            pnl_lich.BackColor = Color.Black
            lbl_Komu.BackColor = Color.Black
            lbl_Komu.ForeColor = Color.White
            lbl_text.BackColor = Color.Black
            lbl_text.ForeColor = Color.White
            GB_lich.BackColor = Color.Black
            GB_lich.ForeColor = Color.White
            sendmes.BackColor = Color.Black
            sendmes.Image = My.Resources.Send_mail
            Me.BackColor = Color.Black
        End If



        Me.KeyPreview = True
        Dim g As New GraphicsPath
        Dim gr As Graphics = Me.CreateGraphics()
        Dim Frm As Rectangle = pic_send.ClientRectangle
        Frm.Inflate(-1, -1)
        g.AddEllipse(Frm)


        'AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)

        If Not msgsubject = String.Empty Then
            txt_Komu.Text = "Тема: " & msgsubject
            msgsubject = String.Empty
        End If
        Me.Text = "Личное сообщение для: " + frndusrname
    End Sub
    '''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''

    Private Sub sendmes_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles sendmes.Click
        'Отправка личного сообщения
        If txt_Komu.Text = String.Empty Then
            MsgBox("Пожалуйста введите тему сообщения!", MsgBoxStyle.Exclamation, "Тема не указана")
            Exit Sub
        Else
            If txt_Komu.Text = String.Empty Then
                MsgBox("Пожалуйста введите корректное сообщение", MsgBoxStyle.Exclamation, "Пустое сообщение")
                Exit Sub
            Else
                Dim sendcommand As New MySqlCommand("INSERT INTO tb_pm (username, fromuser, subject, message, isread) VALUES('" & frndusrname & "', '" & My.Settings.Username & "', '" & txt_Komu.Text & "', '" & txt_text.Text & "', '0')", connection)

                Try
                    sendcommand.ExecuteNonQuery()
                    txt_Komu.Clear()
                    txt_text.Clear()

                    'Запрет на отправку сообщений (Cooldown)  
                    Index = 0
                    pic_send.Visible = True
                    Cooldown_tmr.Start()
                    sendmes.Enabled = False

                Catch ex As Exception
                    MsgBox("Нет соединения с сервером", MsgBoxStyle.Critical, "Ошибка соединения с БД")
                End Try
            End If
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

    Private Sub frm_PrivateMessage_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_PrivateMessage_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub Cooldown_tmr_Tick(sender As Object, e As EventArgs) Handles Cooldown_tmr.Tick
        'Время отправки между сообщениями (откат Cooldown'a)
        Index = Index + 1
        If Index = 6 Then
            pic_send.Size = New Size(30, 20)
            pic_send.Visible = False
            sendmes.Enabled = True
            Cooldown_tmr.Stop()
        End If

        If Index = 0 Then
            pic_send.Size = New Size(30, 20)
        End If

        If Index = 3 Then
            pic_send.Size = New Size(65, 20)
        End If

        If Index = 5 Then
            pic_send.Size = New Size(110, 20)
        End If
    End Sub

    Private Sub frm_PrivateMessage_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class