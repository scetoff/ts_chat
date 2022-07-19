
Imports MySql.Data.MySqlClient
Public Class frm_profilepicture
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Private Sub frm_auth_MouseDown(ByVal sender As Object, ByVal e As MouseEventArgs) Handles MyBase.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then
            Me.BackColor = Color.White
            btn_change.BackColor = Color.White
            btn_change.ForeColor = Color.Black
            ProfilePicture.BackColor = Color.Black
        Else

            'Тёмная тема
            Me.BackColor = Color.Black
            btn_change.BackColor = Color.Black
            btn_change.ForeColor = Color.White
            ProfilePicture.BackColor = Color.White
        End If
    End Sub
    Private Sub btn_change_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles btn_change.Click

        'Проверка выбранного изображения в качестве аватарки
        If imgchoose.ShowDialog = vbOK Then
            Try
                My.Settings.Picture = imgtobytes(imgchoose.FileName)
                My.Settings.Save()

                Dim updatepic As New MySqlCommand("UPDATE tb_users SET dp='" & My.Settings.Picture & "' WHERE username='" & My.Settings.Username & "'", connection)
                updatepic.ExecuteNonQuery()
                MsgBox("Фотография профиля обновлена. Настройки применятся при след. входе.", MsgBoxStyle.Information, "Операция успешно выполнена")
            Catch ex As Exception
                If ex.Message.Contains("Изображение слишком большое") Then
                    MsgBox("Размеры изображения слишком велики для хранения в базе данных. Пожалуйста, используйте изображение меньших размеров, например 250x250.", MsgBoxStyle.Critical, "Недопустимые размеры изображения")
                Else
                    MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка БД")
                End If
            End Try
        End If
    End Sub

    Private Sub frm_profilepicture_Load(ByVal sender As System.Object, ByVal e As System.EventArgs)
        Me.KeyPreview = True

        'Загрузка изображения
        Dim mems As System.IO.MemoryStream = New System.IO.MemoryStream(Convert.FromBase64String(My.Settings.Picture))
        ProfilePicture.Image = Drawing.Image.FromStream(mems)
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

    Private Sub image_dp_Load() Handles MyBase.Load
        'AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)
        displayDP()
    End Sub

    Private Sub displayDP()
        Dim mems As System.IO.MemoryStream = New System.IO.MemoryStream(Convert.FromBase64String(My.Settings.Picture))
        ProfilePicture.Image = Drawing.Image.FromStream(mems)
    End Sub
    Private Sub btn_change_Click() Handles btn_change.Click
        If imgchoose.ShowDialog = vbOK Then
            Try
                Dim updatepic As New MySqlCommand("UPDATE tb_users SET dp='" & My.Settings.Picture & "' WHERE username='" & My.Settings.Username & "'", connection)
                updatepic.ExecuteNonQuery()
                My.Settings.Picture = imgtobytes(imgchoose.FileName)
                My.Settings.Save()
                displayDP()
            Catch ex As Exception
                If ex.Message.Contains("Ошибка") Then
                    MsgBox("Выбранное изображение слишком велико. Пожалуйста используйте изображение размером 640x480.", MsgBoxStyle.Critical, "Недопустимые размеры изображения")
                ElseIf (ex.Message = "Соединение должно быть октрытым.") Then
                    MsgBox(ex.Message + vbNewLine + "Потеря соединения с БД. Попробуйте снова.", MsgBoxStyle.Critical, "Соединение потеряно")
                    Close()
                Else
                    MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
                End If
            End Try
        End If
    End Sub

    Private Sub frm_profilepicture_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class