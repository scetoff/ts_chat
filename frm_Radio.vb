'В качестве радио для чата был использован https://ds-release.ru/primer-onlajn-radio-v2/

'Плагины для работы портируются некорректно, поэтому форма НЕ РАБОТАЕТ, редактирование строго через исходный код
Public Class frm_Radio
    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then
            Me.BackColor = Color.White
            Label1.BackColor = Color.White
            Label1.ForeColor = Color.Black
            ListBox1.BackColor = Color.White
            ListBox1.ForeColor = Color.Black
            Panel1.BackColor = Color.White
            Panel1.ForeColor = Color.Black
            TrackBar1.BackColor = Color.White
            TrackBar1.ForeColor = Color.Black
            AxWindowsMediaPlayer1.Visible = False

            PictureBox1.BackColor = Color.White
            PictureBox2.BackColor = Color.White
            PictureBox3.BackColor = Color.White
            PictureBox4.BackColor = Color.White
            PictureBox5.BackColor = Color.White
            PictureBox9.BackColor = Color.White

            PictureBox2.Image = My.Resources.sound_on_b
            PictureBox3.Image = My.Resources.start_b
            PictureBox4.Image = My.Resources.stop_b
            PictureBox5.Image = My.Resources.pause_b
            PictureBox9.Image = My.Resources.sound_off_b
        Else

            'Тёмная тема
            Me.BackColor = Color.Black
            Label1.BackColor = Color.Black
            Label1.ForeColor = Color.White
            ListBox1.BackColor = Color.Black
            ListBox1.ForeColor = Color.White
            Panel1.BackColor = Color.Black
            Panel1.ForeColor = Color.White
            TrackBar1.BackColor = Color.Black
            TrackBar1.ForeColor = Color.White

            PictureBox1.BackColor = Color.Black
            PictureBox2.BackColor = Color.Black
            PictureBox3.BackColor = Color.Black
            PictureBox4.BackColor = Color.Black
            PictureBox5.BackColor = Color.Black
            PictureBox9.BackColor = Color.Black

            PictureBox2.Image = My.Resources.sound_on
            PictureBox3.Image = My.Resources.start
            PictureBox4.Image = My.Resources._stop
            PictureBox5.Image = My.Resources.pause
            PictureBox9.Image = My.Resources.sound_off
            AxWindowsMediaPlayer1.Visible = True
        End If
    End Sub

    Private Sub trei_Click(sender As Object, e As EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub EndIt_Click(sender As Object, e As EventArgs) Handles EndIt.Click
        Me.Close()
    End Sub

    Private Sub frm_Radio_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_Radio_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
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

    Dim строка As Integer = -1 ' Объявляем номера строк в ListBox1 и присваеваем ему отрицательное значение
    Dim URL As String 'Переменная где будет храниться ссылка для радио
    Private Sub Form1_Load(sender As Object, e As EventArgs) Handles MyBase.Load
        Me.KeyPreview = True
        TrackBar1.Value = 100 'Начальная громкость
    End Sub

    Private Sub PictureBox2_Click(sender As Object, e As EventArgs)
        ' Пункт меню Вкл.Звук
        AxWindowsMediaPlayer1.settings.mute = False
        PictureBox2.Visible = True 'Меняем кнопки местами
        PictureBox9.Visible = False 'Меняем кнопки местами
    End Sub
    Private Sub PictureBox7_Click(sender As Object, e As EventArgs)
        PictureBox2.Visible = True 'Меняем кнопки местами
        PictureBox9.Visible = False 'Меняем кнопки местами
    End Sub

    Private Sub PictureBox6_Click(sender As Object, e As EventArgs)
        AxWindowsMediaPlayer1.settings.mute = False
        PictureBox2.Visible = True 'Меняем кнопки местами
        PictureBox9.Visible = False 'Меняем кнопки местами
    End Sub

    Private Sub Label1_Click(sender As Object, e As EventArgs) Handles Label1.Click
        Label1.Text = "100 %"
    End Sub

    Private Sub PictureBox9_Click(sender As Object, e As EventArgs)
        ' Пункт меню Выкл.Звук
        AxWindowsMediaPlayer1.settings.mute = True
        PictureBox2.Visible = False 'Меняем кнопки местами
        PictureBox9.Visible = True 'Меняем кнопки местами
    End Sub

    Private Sub TrackBar1_Scroll(sender As Object, e As EventArgs) Handles TrackBar1.Scroll
        ' установка уровня звука
        Me.AxWindowsMediaPlayer1.settings.volume = TrackBar1.Value
        Me.Label1.Text = Math.Round(TrackBar1.Value / (TrackBar1.Maximum - TrackBar1.Minimum) * 100 + 0, 1) & "%"
    End Sub

    'Включить радио, пауза, выключить
    Private Sub PictureBox3_Click_1(sender As Object, e As EventArgs) Handles PictureBox3.Click
        AxWindowsMediaPlayer1.Ctlcontrols.play()
    End Sub

    Private Sub PictureBox4_Click_1(sender As Object, e As EventArgs) Handles PictureBox4.Click
        AxWindowsMediaPlayer1.Ctlcontrols.pause()
    End Sub

    Private Sub PictureBox5_Click(sender As Object, e As EventArgs) Handles PictureBox5.Click
        AxWindowsMediaPlayer1.Ctlcontrols.stop()
    End Sub

    Private Sub ListBox1_SelectedIndexChanged_1(sender As Object, e As EventArgs) Handles ListBox1.SelectedIndexChanged
        строка = ListBox1.SelectedIndex + 1 'Номер строки в Названия_радио
        'Считываем URL радио станции с текстового документа.
        Dim Чтение As New System.IO.StreamReader("База\URL.txt")
        Dim i As Integer
        For i = 1 To строка
            URL = Чтение.ReadLine
        Next
        Чтение.Close()
        AxWindowsMediaPlayer1.URL = URL 'Запускаем радио поток
    End Sub

    Private Sub frm_Radio_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub
End Class


