Imports MySql.Data.MySqlClient
Public Class frm_LS
    Dim messages(2000) As String

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
            Me.BackColor = Color.White
            lstmsgs.BackColor = Color.White
            lstmsgs.ForeColor = Color.Black
        Else

            'Тёмная тема
            Me.BackColor = Color.Black
            lstmsgs.BackColor = Color.Black
            lstmsgs.ForeColor = Color.White

        End If
    End Sub
    Private Sub frm_LS_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        Me.KeyPreview = True

        Me.Text += " - " & My.Settings.FirstName
        tempconopen()
        Dim cmdreadmsgs As New MySqlCommand("SELECT * FROM tb_pm WHERE username='" & My.Settings.Username & "'", tempcon)
        Dim msgreader As MySqlDataReader
        Try
            msgreader = cmdreadmsgs.ExecuteReader
        Catch ex As Exception
            If (ex.Message = "Нет соединения с интенетом.") Then
                MsgBox(ex.Message + vbNewLine + "Потеряно соединение с базой данных. Попробуйте еще раз.", MsgBoxStyle.Critical, "Соединение потеряно")
                Close()
            Else
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка БД")
                Exit Sub
            End If
        End Try

        Dim index As Integer = 0
        While msgreader.Read()
            lstmsgs.Items.Add(msgreader(2).ToString).SubItems.Add(msgreader(3).ToString)
            If msgreader(5).ToString = "0" Then

                lstmsgs.Items.Item(index).Font = New Font(lstmsgs.Font.FontFamily, lstmsgs.Font.Size, FontStyle.Bold)
            End If
            messages(index) = msgreader(4).ToString
            index += 1
        End While
        msgreader.Close()
        tempcon.Close()
    End Sub

    Private Sub lstmsgs_SelectedIndexChanged(sender As Object, e As MouseEventArgs) Handles lstmsgs.MouseClick
        'Список личных сообщений
        If e.Button = Windows.Forms.MouseButtons.Left Then
            If Not lstmsgs.FocusedItem.Index = -1 Then
                Dim update As New MySqlCommand("UPDATE tb_pm SET isread='1' WHERE subject='" & lstmsgs.FocusedItem.SubItems(1).Text & "'", connection)
                Try
                    update.ExecuteNonQuery()
                Catch ex As Exception
                    If (ex.Message = "Нет соединения с интенетом.") Then
                        MsgBox(ex.Message + vbNewLine + "Потеряно соединение с базой данных. Попробуйте еще раз.", MsgBoxStyle.Critical, "Соединение потеряно")
                        Exit Sub
                    Else
                        MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка БД")
                        Exit Sub
                    End If
                End Try

                'Подробные сведения о сообщении
                lstmsgs.Items.Item(lstmsgs.FocusedItem.Index).Font = New Font(lstmsgs.Font.FontFamily, lstmsgs.Font.Size, FontStyle.Regular)
                Dim reply = MsgBox("Для: " & My.Settings.Username & vbNewLine & vbNewLine & "От: " & lstmsgs.FocusedItem.Text & vbNewLine & vbNewLine & "Тема: " & lstmsgs.FocusedItem.SubItems(1).Text & vbNewLine & vbNewLine & "Сообщение: " & messages(lstmsgs.FocusedItem.Index) & vbNewLine & vbNewLine & "Вы хотите ответить?", MsgBoxStyle.YesNo, "Личное сообщение")
                If reply = vbYes Then
                    Dim pmform As New frm_PrivateMessage
                    msgsubject = lstmsgs.FocusedItem.SubItems(1).Text
                    frndusrname = lstmsgs.FocusedItem.Text
                    pmform.ShowDialog()

                End If
            End If
        End If
    End Sub

    Private Sub frm_LS_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_LS_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
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

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Private Sub Delmsg_Click(sender As Object, e As EventArgs) Handles delmsg.Click

        'Удаление личного сообщения
        If Not lstmsgs.FocusedItem.Index = -1 Then
            Dim deletemsg As New MySqlCommand("DELETE FROM tb_pm WHERE subject='" & lstmsgs.FocusedItem.SubItems(1).Text & "' AND username='" & My.Settings.Username & "'", connection)
            Try
                deletemsg.ExecuteNonQuery()
            Catch ex As Exception
                If (ex.Message = "Недоступно соединение с БД.") Then
                    MsgBox(ex.Message + vbNewLine + "Потеряно соединение с БД. Попробуйте позже", MsgBoxStyle.Critical, "Соединение потеряно")
                    Exit Sub
                Else
                    MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка БД")
                    Exit Sub
                End If
            End Try
            lstmsgs.FocusedItem.Remove()
        End If
    End Sub

    Private Sub frm_LS_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class