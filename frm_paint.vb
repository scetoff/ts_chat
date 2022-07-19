Imports System.Drawing
Public Class frm_paint
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean
    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    Dim G As Graphics
    Dim B As New SolidBrush(Color.Black)

    Dim down = False
    Dim mybrush = Brushes.Black

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim BB As Integer

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then
            Me.BackColor = Color.White
            Menu_StripLine.BackColor = Color.White
            Menu_StripLine.ForeColor = Color.Black
            Menu_StripLine.BackgroundImage = My.Resources.white
            btn_color.BackColor = Color.White
            btn_color.ForeColor = Color.Black
            ФайлToolStripMenuItem.ForeColor = Color.Black
            ЗаливкаToolStripMenuItem.ForeColor = Color.Black
        Else

            'Тёмная тема
            Me.BackColor = Color.Black
            Menu_StripLine.BackColor = Color.Black
            Menu_StripLine.ForeColor = Color.White
            Menu_StripLine.BackgroundImage = My.Resources.black
            btn_color.BackColor = Color.Black
            btn_color.ForeColor = Color.White
            ФайлToolStripMenuItem.ForeColor = Color.White
            ЗаливкаToolStripMenuItem.ForeColor = Color.White
        End If
    End Sub

    Private Sub trei_Click(sender As Object, e As EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(sender As Object, e As EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Private Sub frm_paint_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        BB = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_paint_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (BB)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub Stringer_MouseDown(sender As Object, e As MouseEventArgs) Handles Stringer.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        BB = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub Stringer_MouseMove(sender As Object, e As MouseEventArgs) Handles Stringer.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (BB)
            Me.Location = newPoint
        End If
    End Sub
    Private Sub image_box_MouseMove(sender As Object, e As MouseEventArgs) Handles image_box.MouseMove

        'Размер кисти
        If down = True Then
            image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 14, 14)
        End If

        If cbox_size.SelectedItem = "16" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 16, 16)
            End If
        End If

        If cbox_size.SelectedItem = "18" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 18, 18)
            End If
        End If

        If cbox_size.SelectedItem = "20" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 20, 20)
            End If
        End If

        If cbox_size.SelectedItem = "32" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 32, 32)
            End If
        End If

        If cbox_size.SelectedItem = "36" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 36, 36)
            End If
        End If

        If cbox_size.SelectedItem = "48" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 48, 48)
            End If
        End If

        If cbox_size.SelectedItem = "72" Then
            If down = True Then
                image_box.CreateGraphics.FillEllipse(mybrush, e.X, e.Y, 72, 72)
            End If
        End If
    End Sub

    Private Sub image_box_MouseUp(sender As Object, e As MouseEventArgs) Handles image_box.MouseUp
        down = False
    End Sub

    Private Sub image_box_MouseDown(sender As Object, e As MouseEventArgs) Handles image_box.MouseDown
        down = True
    End Sub

    Private Sub btn_color_Click(sender As Object, e As EventArgs) Handles btn_color.Click
        ColorDialog1.ShowDialog()
        mybrush.color = ColorDialog1.Color 'Заливка фона
    End Sub

    Private Sub ЗаливкаToolStripMenuItem_Click(sender As Object, e As EventArgs) Handles ЗаливкаToolStripMenuItem.Click
        ColorDialog1.ShowDialog()
        image_box.BackColor = ColorDialog1.Color 'Изменение цвета кисти
    End Sub


    Private Sub СохранитьToolStripMenuItem_Click(sender As Object, e As EventArgs) Handles СохранитьToolStripMenuItem.Click
        'image_box.Image.Save("C:/")
    End Sub

    Private Sub frm_paint_Load(sender As Object, e As EventArgs) Handles Me.Load
        Me.KeyPreview = True
        AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)
    End Sub


    Private Sub frm_paint_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class