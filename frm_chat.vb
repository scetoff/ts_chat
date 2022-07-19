Imports MySql.Data.MySqlClient.MySqlCommand
Imports System.Collections
Imports MySql.Data.MySqlClient
Imports System.Threading
Imports System.Globalization

Imports System.Speech
Imports System
Imports System.Collections.Generic
Imports System.ComponentModel
Imports System.Data
Imports System.Drawing
Imports System.Linq
Imports System.Text
Imports System.Windows.Forms
Imports Microsoft.Speech.Recognition

Public Class frm_chat

    Public Sub New()
        InitializeComponent()

        'Светлая тема
        If frm_Auth.Checker.Checked = True Then

            Me.BackColor = Color.White
            Rtxt_chat.BackColor = Color.White
            Rtxt_chat.ForeColor = Color.Black
            GB_Smails.BackColor = Color.White
            GB_Smails.ForeColor = Color.Black
            GB_Xz.BackColor = Color.White
            GB_Xz.ForeColor = Color.Black
            logo.BackColor = Color.White
            lst_friends.BackColor = Color.White
            lst_friends.ForeColor = Color.Black
            GB_XZ2.BackColor = Color.White
            GB_XZ2.ForeColor = Color.Black
            Username.BackColor = Color.White
            Username.ForeColor = Color.Black
            phonework.BackColor = Color.White
            phonework.ForeColor = Color.Black
            mailes.BackColor = Color.White
            mailes.ForeColor = Color.Black
            Avatar.BackColor = Color.White
            sendmes.BackColor = Color.White
            pctr_prikrep.BackColor = Color.White
            smile_fon.BackColor = Color.White
            history.BackColor = Color.White
            history.ForeColor = Color.Black
            MenuStrip1.BackColor = Color.White
            MenuStrip1.ForeColor = Color.Black
            MenuStrip1.BackgroundImage = My.Resources.white

            ToolStripMenuItem18.BackColor = Color.White
            ToolStripMenuItem18.ForeColor = Color.Black
            РедактироватьПрофильToolStripMenuItem1.BackColor = Color.White
            РедактироватьПрофильToolStripMenuItem1.ForeColor = Color.Black
            ToolStripMenuItem20.BackColor = Color.White
            ToolStripMenuItem20.ForeColor = Color.Black
            ВыбратьЦветToolStripMenuItem1.BackColor = Color.White
            ВыбратьЦветToolStripMenuItem1.ForeColor = Color.Black
            ВыбратьШрифтToolStripMenuItem1.BackColor = Color.White
            ВыбратьШрифтToolStripMenuItem1.ForeColor = Color.Black
            ToolStripMenuItem23.BackColor = Color.White
            ToolStripMenuItem23.ForeColor = Color.Black
            ToolStripMenuItem25.BackColor = Color.White
            ToolStripMenuItem25.ForeColor = Color.Black
            РадиоToolStripMenuItem1.BackColor = Color.White
            РадиоToolStripMenuItem1.ForeColor = Color.Black
            ЗаметкиToolStripMenuItem.BackColor = Color.White
            ЗаметкиToolStripMenuItem.ForeColor = Color.Black
            ToolStripMenuItem28.BackColor = Color.White
            ToolStripMenuItem28.ForeColor = Color.Black
            ToolStripMenuItem24.BackColor = Color.White
            ToolStripMenuItem24.ForeColor = Color.Black

            ToolStripMenuItem18.Image = My.Resources.Fail_b
            РедактироватьПрофильToolStripMenuItem1.Image = My.Resources.redakt_b
            sendmes.Image = My.Resources.Send_mail_b
            pctr_prikrep.Image = My.Resources.prikrep_b
            profile.Image = My.Resources.profile_b
            ToolStripMenuItem20.Image = My.Resources.fontes_b
            ВыбратьШрифтToolStripMenuItem1.Image = My.Resources.fontB_b
            ToolStripMenuItem23.Image = My.Resources.clear_b
            РадиоToolStripMenuItem1.Image = My.Resources.radio_b
            ЗаметкиToolStripMenuItem.Image = My.Resources.pencil_b
            ToolStripMenuItem28.Image = My.Resources.icq_lgtp_b
            ToolStripMenuItem24.Image = My.Resources.lmseg_b

        Else
            'Тёмная тема

            Me.BackColor = Color.Black
            Rtxt_chat.BackColor = Color.Black
            Rtxt_chat.ForeColor = Color.White
            GB_Smails.BackColor = Color.Black
            GB_Smails.ForeColor = Color.White
            GB_Xz.BackColor = Color.Black
            GB_Xz.ForeColor = Color.White
            logo.BackColor = Color.Black
            lst_friends.BackColor = Color.Black
            lst_friends.ForeColor = Color.White
            GB_XZ2.BackColor = Color.Black
            GB_XZ2.ForeColor = Color.White
            Username.BackColor = Color.Black
            Username.ForeColor = Color.White
            phonework.BackColor = Color.Black
            phonework.ForeColor = Color.White
            mailes.BackColor = Color.Black
            mailes.ForeColor = Color.White
            Avatar.BackColor = Color.Black
            sendmes.BackColor = Color.Black
            pctr_prikrep.BackColor = Color.Black
            smile_fon.BackColor = Color.Black
            history.BackColor = Color.Black
            history.ForeColor = Color.White
            MenuStrip1.BackColor = Color.Black
            MenuStrip1.ForeColor = Color.White
            MenuStrip1.BackgroundImage = My.Resources.black

            ToolStripMenuItem18.BackColor = Color.Black
            ToolStripMenuItem18.ForeColor = Color.White
            РедактироватьПрофильToolStripMenuItem1.BackColor = Color.Black
            РедактироватьПрофильToolStripMenuItem1.ForeColor = Color.White
            ToolStripMenuItem20.BackColor = Color.Black
            ToolStripMenuItem20.ForeColor = Color.White
            ВыбратьЦветToolStripMenuItem1.BackColor = Color.Black
            ВыбратьЦветToolStripMenuItem1.ForeColor = Color.White
            ВыбратьШрифтToolStripMenuItem1.BackColor = Color.Black
            ВыбратьШрифтToolStripMenuItem1.ForeColor = Color.White
            ToolStripMenuItem23.BackColor = Color.Black
            ToolStripMenuItem23.ForeColor = Color.White
            ToolStripMenuItem25.BackColor = Color.Black
            ToolStripMenuItem25.ForeColor = Color.White
            РадиоToolStripMenuItem1.BackColor = Color.Black
            РадиоToolStripMenuItem1.ForeColor = Color.White
            ЗаметкиToolStripMenuItem.BackColor = Color.Black
            ЗаметкиToolStripMenuItem.ForeColor = Color.White
            ToolStripMenuItem28.BackColor = Color.Black
            ToolStripMenuItem28.ForeColor = Color.White
            ToolStripMenuItem24.BackColor = Color.Black
            ToolStripMenuItem24.ForeColor = Color.White

            ToolStripMenuItem18.Image = My.Resources.Fail
            РедактироватьПрофильToolStripMenuItem1.Image = My.Resources.redakt
            sendmes.Image = My.Resources.Send_mail
            pctr_prikrep.Image = My.Resources.prikrep
            profile.Image = My.Resources.profile
            ToolStripMenuItem20.Image = My.Resources.fontes
            ВыбратьШрифтToolStripMenuItem1.Image = My.Resources.fontB
            ToolStripMenuItem23.Image = My.Resources.clear
            РадиоToolStripMenuItem1.Image = My.Resources.radio
            ЗаметкиToolStripMenuItem.Image = My.Resources.pencil
            ToolStripMenuItem28.Image = My.Resources.icq_lgtp
            ToolStripMenuItem24.Image = My.Resources.lmseg
        End If
    End Sub

    Sub SpeechRecognized(sender As Object, e As SpeechRecognizedEventArgs)

        'Распознавание выбранной команды
        Select Case e.Result.Text

            Case "Включи первую станцию"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 0
                Else
                    frm_Radio.ListBox1.SelectedIndex = 0
                End If

            Case "Включи Русский рок", "Включи рок"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 1
                Else
                    frm_Radio.ListBox1.SelectedIndex = 1
                End If

            Case "Включи Европа плюс", "Включи Европу"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 2
                Else
                    frm_Radio.ListBox1.SelectedIndex = 2
                End If

            Case "Включи Радио кафе", "Включи Кафе"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 3
                Else
                    frm_Radio.ListBox1.SelectedIndex = 3
                End If

            Case "Включи Дорожное радио"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 4
                Else
                    frm_Radio.ListBox1.SelectedIndex = 4
                End If

            Case "Включи Старое радио", "Включи Детское радио"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 5
                Else
                    frm_Radio.ListBox1.SelectedIndex = 5
                End If

            Case "Включи Вип Микс радио", "Включи вип микс"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 6
                Else
                    frm_Radio.ListBox1.SelectedIndex = 6
                End If

            Case "Включи рекорд дабстеп", "Включи дабстеп"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 7
                Else
                    frm_Radio.ListBox1.SelectedIndex = 7
                End If

            Case "Включи Чилаут радио", "Включи чилаут"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 8
                Else
                    frm_Radio.ListBox1.SelectedIndex = 8
                End If

            Case "Включи радио Джаз", "Включи Джаз"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 9
                Else
                    frm_Radio.ListBox1.SelectedIndex = 9
                End If

            Case "Включи Радио Книга", "Включи книгу", "Включи аудиокнигу"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 10
                Else
                    frm_Radio.ListBox1.SelectedIndex = 10
                End If

            Case "Включи дискотеку девяностых", "Включи дискотеку"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 11
                Else
                    frm_Radio.ListBox1.SelectedIndex = 11
                End If

            Case "Включи амбиэнт", "Включи слипинг пил"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 12
                Else
                    frm_Radio.ListBox1.SelectedIndex = 12
                End If

            Case "Включи драм энд бэйс", "Включи драм"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 13
                Else
                    frm_Radio.ListBox1.SelectedIndex = 13
                End If

            Case "Включи Тектоник"
                If frm_Radio.Visible = False Then
                    frm_Radio.Show()
                    frm_Radio.ListBox1.SelectedIndex = 14
                Else
                    frm_Radio.ListBox1.SelectedIndex = 14
                End If

            Case "Выключи радио", "Отключи радио"
                frm_Radio.AxWindowsMediaPlayer1.Ctlcontrols.stop()

            Case "Включи радио"
                frm_Radio.AxWindowsMediaPlayer1.Ctlcontrols.play()

            Case "Закрой радио"
                frm_Radio.Close()

            Case "Открой радио", "Запусти радио"
                frm_Radio.Show()

            Case "Открой заметки"
                frm_paint.Show()

            Case "Закрой заметки"
                frm_paint.Close()

            Case "Очисти чат", "Очисти весь чат"
                ToolStripMenuItem23_Click(ToolStripMenuItem23, Nothing)
        End Select
    End Sub

    Private Sub frm_chat_Shown(sender As Object, e As EventArgs) Handles Me.Shown

        Dim ci As New System.Globalization.CultureInfo("ru-RU")
        Dim sre As New SpeechRecognitionEngine(ci)
        sre.SetInputToDefaultAudioDevice()
        AddHandler sre.SpeechRecognized, AddressOf SpeechRecognized

        'Перечень всех доступных команд
        Dim texts As New Choices()
        texts.Add(New String() {"Включи первую станцию", "Включи Европа плюс", "Включи Европу", "Включи Русский рок", "Включи рок", "Включи Радио кафе", "Включи Кафе",
                  "Включи Дорожное радио", "Включи Старое радио", "Включи Детское радио", "Включи Вип Микс радио", "Включи вип микс", "Включи рекорд дабстеп", "Включи дабстеп",
                  "Включи Чилаут радио", "Включи чилаут", "Включи радио Джаз", "Включи Джаз", "Включи Радио Книга", "Включи книгу", "Включи аудиокнигу", "Включи амбиэнт", "Включи слипинг пил", "Включи драм энд бэйс", "Включи драм",
                  "Выключи радио", "Включи дискотеку девяностых", "Включи дискотеку", "Включи Тектоник", "Включи радио", "Выключи радио", "Отключи радио", "Закрой радио", "Открой радио", "Запусти радио",
                  "Открой заметки", "Закрой заметки", "Очисти чат", "Очисти весь чат"})
        Dim gb As New GrammarBuilder()
        gb.Culture = ci
        gb.Append(texts)
        Dim g As New Grammar(gb)
        sre.LoadGrammar(g)
        sre.RecognizeAsync(RecognizeMode.Multiple)
    End Sub

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    'Загрузка шрифтов для чата
    Dim shrifty As New System.Drawing.Text.InstalledFontCollection
    Dim messages(2000) As String 'Лимит на количество символов в чате


    Private Sub frm_chat_FormClosing(ByVal sender As Object, ByVal e As System.Windows.Forms.FormClosingEventArgs) Handles Me.FormClosing
        frm_Auth.Show()
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
        Me.Close()
    End Sub

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub
    Private Sub frm_chat_Load(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles MyBase.Load
        Me.KeyPreview = True

        GB_Smails.Visible = False
        Me.Text = Me.Text & " - " & My.Settings.FirstName
        Dim mems As System.IO.MemoryStream = New System.IO.MemoryStream(Convert.FromBase64String(My.Settings.Picture))
        Avatar.Image = Drawing.Image.FromStream(mems)
        Username.Text = My.Settings.FirstName
        mailes.Text = My.Settings.Email
        phonework.Text = My.Settings.telPhone
        friends()
        CreateEmotions()

        ChatBoxBackup.Text = "Добро пожаловать в Таверну Странника"

        Me.Width = 497
        Me.EndIt.Location = New Point(462, 0)
        Me.trei.Location = New Point(421, 0)
    End Sub

    Private Sub YourPicture_click() Handles Avatar.Click 'Выбор аватарки
        Dim pic As New frm_profilepicture
        pic.ShowDialog()
    End Sub

    Private Sub friends() 'Список друзей
        Dim readcommand As New MySqlCommand("SELECT * FROM tb_users", connection)
        Dim datareader As MySqlDataReader
        datareader = readcommand.ExecuteReader

        Dim chatfrom As New frm_chat
        While datareader.Read()

            If Not datareader(1).ToString = My.Settings.Username Then
                lst_friends.Items.Add(datareader(1).ToString)
            End If
        End While
        datareader.Close()
    End Sub

    Private Sub lst_Friends_selecteditem() Handles lst_friends.Click 'Информация о друге

        If Not lst_friends.SelectedIndex = -1 Then
            Dim friends As New frm_FriendsProfile
            frndusrname = lst_friends.SelectedItem.ToString
            friends.ShowDialog()
        End If
    End Sub

    Private Sub tmr_chat_Tick() Handles tmr_chat.Tick

        Dim cmdreadchat As New MySqlCommand("SELECT chat FROM tb_chat", connection)
        Dim chatreader As MySqlDataReader
        chatreader = cmdreadchat.ExecuteReader

        While chatreader.Read()
            If Not chatreader(0).ToString = ChatBoxBackup.Text Then
                ChatBoxBackup.Text = chatreader(0).ToString
            End If
        End While
        chatreader.Close()
    End Sub

    Private Sub Rtxt_chat_textchanged() Handles Rtxt_chat.TextChanged 'Просмотр чата
        Rtxt_chat.ScrollToCaret()
    End Sub

    Private Sub Rtxt_chat_textchanged(ByVal sender As Object, ByVal e As EventArgs) Handles Rtxt_chat.TextChanged
        Rtxt_chat.SelectionStart() = Rtxt_chat.TextLength
        Rtxt_chat.ScrollToCaret()
    End Sub

    Private Sub profile_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles profile.Click
        ' Дополнительная вкладка
        If Me.Width = 497 Then
            Me.Width = 796
            Me.EndIt.Location = New Point(757, 0)
            Me.trei.Location = New Point(716, 0)
        Else
            Me.Width = 497
            Me.EndIt.Location = New Point(462, 0)
            Me.trei.Location = New Point(421, 0)
        End If
    End Sub

    Private Sub sendmes_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles sendmes.Click
        'Отправка сообщения
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ":  " & history.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()

        history.Clear()
    End Sub

    Private Sub ВыбратьШрифтToolStripMenuItem_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ВыбратьШрифтToolStripMenuItem.Click
        'Выбор шрифтов
        If FontS.ShowDialog = vbOK Then
            My.Settings.ChatFont = FontS.Font.FontFamily.ToString.Remove(0, 18).Replace("]", "")
            My.Settings.ChatFontSize = CInt(FontS.Font.Size)
            If FontS.Font.Bold = True Then
                My.Settings.ChatFontStyle = "Bold"
            ElseIf FontS.Font.Italic = True Then
                My.Settings.ChatFontStyle = "Italic"
            ElseIf FontS.Font.Underline = True Then
                My.Settings.ChatFontStyle = "Underline"
            ElseIf FontS.Font.Strikeout = True Then
                My.Settings.ChatFontStyle = "Strikeout"
            End If
        End If
    End Sub



    Private Sub ВыбратьЦветToolStripMenuItem1_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ВыбратьЦветToolStripMenuItem1.Click
        'Выбор цвета никнейма
        If Colors.ShowDialog = vbOK Or vbCancel Then
            My.Settings.ChatColor = Colors.Color.ToArgb.ToString("X")
        End If
        My.Settings.Save()
        Dim updatefont As New MySqlCommand("UPDATE tb_users SET chatcolor='" & My.Settings.ChatColor & "', chatfont='" & My.Settings.ChatFont & "', chatfontsize='" & My.Settings.ChatFontSize & "', chatfontstyle='" & My.Settings.ChatFontStyle & "' WHERE username='" & My.Settings.Username & "'", connection)
        updatefont.ExecuteNonQuery()
        changefontcolor()

    End Sub

    Private Sub changefontcolor() 'Сохранение информации о выбранном цвете
        tempconopen()
        Dim ReadUser As New MySqlCommand("SELECT * FROM tb_users", tempcon)
        Dim UserReadd As MySqlDataReader
        UserReadd = ReadUser.ExecuteReader

        While UserReadd.Read()
            Dim ind As Integer = 0
            While Not ind = -1
                ind = Rtxt_chat.Find(UserReadd(1).ToString, ind + 1, RichTextBoxFinds.WholeWord)
                Dim startindex As Integer
                Dim lengthofline As Integer
                For i As Integer = 0 To Rtxt_chat.Lines.Count - 1
                    If ind = Rtxt_chat.GetFirstCharIndexFromLine(i) Then
                        startindex = ind
                        lengthofline = Rtxt_chat.Lines(i).Length
                        Exit For
                    End If
                Next
                Rtxt_chat.Select(startindex, lengthofline)
                Rtxt_chat.SelectionColor = Color.FromArgb(Convert.ToInt32(UserReadd(7).ToString, 16))
                Rtxt_chat.Select(startindex, lengthofline)
                If UserReadd(10).ToString = "Bold" Then
                    Rtxt_chat.SelectionFont = New Font(UserReadd(8).ToString, CInt(UserReadd(9).ToString), FontStyle.Bold)
                ElseIf UserReadd(10).ToString = "Italic" Then
                    Rtxt_chat.SelectionFont = New Font(UserReadd(8).ToString, CInt(UserReadd(9).ToString), FontStyle.Italic)
                ElseIf UserReadd(10).ToString = "Underline" Then
                    Rtxt_chat.SelectionFont = New Font(UserReadd(8).ToString, CInt(UserReadd(9).ToString), FontStyle.Underline)
                ElseIf UserReadd(10).ToString = "Strikeout" Then
                    Rtxt_chat.SelectionFont = New Font(UserReadd(8).ToString, CInt(UserReadd(9).ToString), FontStyle.Strikeout)
                Else
                    Rtxt_chat.SelectionFont = New Font(UserReadd(8).ToString, CInt(UserReadd(9).ToString), FontStyle.Regular)
                End If

            End While
        End While
        UserReadd.Close()
        tempcon.Close()
    End Sub

    Private emotions As Hashtable

    Private Sub AddEmotions() 'Отправка смайлов в чат
        For Each emote As String In emotions.Keys
            While Rtxt_chat.Text.Contains(emote)
                Dim ind As Integer = Rtxt_chat.Text.IndexOf(emote)
                Rtxt_chat.Select(ind, emote.Length)
                Clipboard.SetImage(DirectCast(emotions(emote), Image))
                Rtxt_chat.Paste()
            End While
        Next
    End Sub

    Private Sub ChatBoxBackup_TextChanged(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles ChatBoxBackup.TextChanged
        Rtxt_chat.Text = ChatBoxBackup.Text
        changefontcolor()
        Rtxt_chat.ReadOnly = False
        AddEmotions()
        Rtxt_chat.ReadOnly = True
    End Sub

    Private Sub pctr_prikrep_Click(sender As Object, e As EventArgs) Handles pctr_prikrep.Click

        'OpenFileDialog2.ShowDialog()
    End Sub
    Private Sub smile_fon_Click(sender As Object, e As EventArgs) Handles smile_fon.Click
        'Показать панель со смайлами
        If GB_Smails.Visible = False Then
            GB_Smails.Visible = True
        Else
            GB_Smails.Visible = False
        End If
    End Sub
    Private Sub ВыбратьШрифтToolStripMenuItem1_Click(sender As Object, e As EventArgs) Handles ВыбратьШрифтToolStripMenuItem1.Click
        If FontS.ShowDialog = vbOK Then
            My.Settings.ChatFont = FontS.Font.FontFamily.ToString.Remove(0, 18).Replace("]", "")
            My.Settings.ChatFontSize = CInt(FontS.Font.Size)
            If FontS.Font.Bold = True Then
                My.Settings.ChatFontStyle = "Bold"
            ElseIf FontS.Font.Italic = True Then
                My.Settings.ChatFontStyle = "Italic"
            ElseIf FontS.Font.Underline = True Then
                My.Settings.ChatFontStyle = "Underline"
            ElseIf FontS.Font.Strikeout = True Then
                My.Settings.ChatFontStyle = "Strikeout"
            End If
        End If
    End Sub

    Private Sub ToolStripMenuItem23_Click(sender As Object, e As EventArgs) Handles ToolStripMenuItem23.Click

        If MsgBox("Вы действительно хотите очистить чат!" & vbNewLine & "Примечание: Это сообщение будет удалено навсегда!", MsgBoxStyle.YesNoCancel, "Очистить весь чат!") = MsgBoxResult.Yes Then
            ChatBoxBackup.Text = "Добро пожаловать в Таверну Странника"
            Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
            updatechat.ExecuteNonQuery()
        End If
    End Sub

    Private Sub РадиоToolStripMenuItem1_Click(sender As Object, e As EventArgs) Handles РадиоToolStripMenuItem1.Click
        frm_Radio.Show()
    End Sub

    Private Sub ЗаметкиToolStripMenuItem_Click(sender As Object, e As EventArgs) Handles ЗаметкиToolStripMenuItem.Click
        frm_paint.Show()
    End Sub

    Private Sub ToolStripMenuItem28_Click(sender As Object, e As EventArgs) Handles ToolStripMenuItem28.Click

        MsgBox("Некоторое время назад была прекращена поддержка плагина для icq, причиной, послужившей этому могли стать брут-программы, но данная информация является не подтверждённой", vbOKOnly, "Предупреждение")

        If frm_icq.Visible = True Then
            MsgBox("Аська уже запущена", vbOKOnly, "Предупреждение")
        Else
            frm_icq.Show()
            frm_chat.ActiveForm.Update()
        End If
    End Sub

    Private Sub РедактироватьПрофильToolStripMenuItem1_Click(sender As Object, e As EventArgs) Handles РедактироватьПрофильToolStripMenuItem1.Click
        Dim profiledit As New frm_EditProfile
        profiledit.ShowDialog()
    End Sub
    Private Sub history_KeyDown(sender As Object, e As KeyEventArgs) Handles history.KeyDown
        'Отправка сообщений через Enter
        If e.KeyCode = Keys.Enter Then
            history.AppendText("")
            e.SuppressKeyPress = True
        End If

        If e.KeyCode = Keys.Enter Then
            e.Handled = True
            ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & history.Text
            Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
            updatechat.ExecuteNonQuery()

            history.Clear()
        Else
            e.Handled = False
        End If
    End Sub

    Private Sub Frm_chat_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub Frm_chat_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub ToolStripMenuItem24_Click() Handles ToolStripMenuItem24.Click
        Dim viewpmform As New frm_LS
        viewpmform.ShowDialog()
    End Sub

    Private Sub frm_chat_KeyDown(sender As Object, e As KeyEventArgs) Handles Me.KeyDown
        If e.KeyCode = Keys.Escape Then Application.Exit()
    End Sub

    'Выгрузка смайлов в чат

    Private Sub PictureBox1_Click(sender As Object, e As EventArgs) Handles PictureBox1.Click
        txtBoxHelper.Text = ":fuu:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox2_Click(sender As Object, e As EventArgs) Handles PictureBox2.Click
        txtBoxHelper.Text = ":beer:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox3_Click(sender As Object, e As EventArgs) Handles PictureBox3.Click
        txtBoxHelper.Text = ":sorry:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox4_Click(sender As Object, e As EventArgs) Handles PictureBox4.Click
        txtBoxHelper.Text = ":read:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox5_Click(sender As Object, e As EventArgs) Handles PictureBox5.Click
        txtBoxHelper.Text = ":girl:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox6_Click(sender As Object, e As EventArgs) Handles PictureBox6.Click
        txtBoxHelper.Text = ":lich:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox7_Click(sender As Object, e As EventArgs) Handles PictureBox7.Click
        txtBoxHelper.Text = ":sing:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox8_Click(sender As Object, e As EventArgs) Handles PictureBox8.Click
        txtBoxHelper.Text = ":haha:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox9_Click(sender As Object, e As EventArgs) Handles PictureBox9.Click
        txtBoxHelper.Text = ":allbear:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox10_Click(sender As Object, e As EventArgs) Handles PictureBox10.Click
        txtBoxHelper.Text = ":say:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox11_Click(sender As Object, e As EventArgs) Handles PictureBox11.Click
        txtBoxHelper.Text = ":money:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox12_Click(sender As Object, e As EventArgs) Handles PictureBox12.Click
        txtBoxHelper.Text = ":no:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox13_Click(sender As Object, e As EventArgs) Handles PictureBox13.Click
        txtBoxHelper.Text = ":king:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox14_Click(sender As Object, e As EventArgs) Handles PictureBox14.Click
        txtBoxHelper.Text = ":go:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox15_Click(sender As Object, e As EventArgs) Handles PictureBox15.Click
        txtBoxHelper.Text = ":good:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub PictureBox16_Click(sender As Object, e As EventArgs) Handles PictureBox16.Click
        txtBoxHelper.Text = ":nice:"
        ChatBoxBackup.Text += vbNewLine & vbNewLine & My.Settings.Username & ": " & txtBoxHelper.Text
        Dim updatechat As New MySqlCommand("UPDATE tb_chat SET chat='" & ChatBoxBackup.Text & "'", connection)
        updatechat.ExecuteNonQuery()
        txtBoxHelper.Text = ""
        GB_Smails.Visible = False
    End Sub

    Private Sub CreateEmotions() 'Отправка смайлов через чат
        emotions = New Hashtable(12)
        emotions.Add(":fuu:", My.Resources._1)
        emotions.Add(":beer:", My.Resources._2)
        emotions.Add(":sorry:", My.Resources._3)
        emotions.Add(":read:", My.Resources._4)
        emotions.Add(":girl:", My.Resources._5)
        emotions.Add(":lich:", My.Resources._6)
        emotions.Add(":sing:", My.Resources._7)
        emotions.Add(":haha:", My.Resources._8)
        emotions.Add(":allbear:", My.Resources._9)
        emotions.Add(":say:", My.Resources._10)
        emotions.Add(":money:", My.Resources._11)
        emotions.Add(":no:", My.Resources._12)
        emotions.Add(":king:", My.Resources._13)
        emotions.Add(":go:", My.Resources._14)
        emotions.Add(":good:", My.Resources._15)
        emotions.Add(":nice:", My.Resources._16)
    End Sub
End Class
