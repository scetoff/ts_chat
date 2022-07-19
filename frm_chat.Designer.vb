<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()>
Partial Class frm_chat
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
    <System.Diagnostics.DebuggerNonUserCode()>
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()>
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_chat))
        Me.BWorker_chat = New System.ComponentModel.BackgroundWorker()
        Me.Colors = New System.Windows.Forms.ColorDialog()
        Me.FontS = New System.Windows.Forms.FontDialog()
        Me.tmr_chat = New System.Windows.Forms.Timer(Me.components)
        Me.GB_Chat = New System.Windows.Forms.GroupBox()
        Me.GB_Smails = New System.Windows.Forms.GroupBox()
        Me.Rtxt_chat = New System.Windows.Forms.RichTextBox()
        Me.ChatBoxBackup = New System.Windows.Forms.RichTextBox()
        Me.GB_Users = New System.Windows.Forms.GroupBox()
        Me.lst_friends = New System.Windows.Forms.ListBox()
        Me.GB_XZ2 = New System.Windows.Forms.GroupBox()
        Me.phonework = New System.Windows.Forms.Label()
        Me.mailes = New System.Windows.Forms.Label()
        Me.Username = New System.Windows.Forms.Label()
        Me.GB_Xz = New System.Windows.Forms.GroupBox()
        Me.history = New System.Windows.Forms.TextBox()
        Me.OpenFileDialog2 = New System.Windows.Forms.OpenFileDialog()
        Me.OpenFileDialog1 = New System.Windows.Forms.OpenFileDialog()
        Me.btn_naps = New System.Windows.Forms.Button()
        Me.Hint_User = New System.Windows.Forms.ToolTip(Me.components)
        Me.РазвлеченияToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.txtBoxHelper = New System.Windows.Forms.TextBox()
        Me.ToolStripMenuItem14 = New System.Windows.Forms.ToolStripMenuItem()
        Me.MenuStrip1 = New System.Windows.Forms.MenuStrip()
        Me.ToolStripMenuItem18 = New System.Windows.Forms.ToolStripMenuItem()
        Me.РедактироватьПрофильToolStripMenuItem1 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem20 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ВыбратьШрифтToolStripMenuItem1 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ВыбратьЦветToolStripMenuItem1 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem23 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem24 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem25 = New System.Windows.Forms.ToolStripMenuItem()
        Me.РадиоToolStripMenuItem1 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ЗаметкиToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem28 = New System.Windows.Forms.ToolStripMenuItem()
        Me.pctr_prikrep = New System.Windows.Forms.PictureBox()
        Me.sendmes = New System.Windows.Forms.PictureBox()
        Me.smile_fon = New System.Windows.Forms.Button()
        Me.Avatar = New System.Windows.Forms.PictureBox()
        Me.PictureBox16 = New System.Windows.Forms.PictureBox()
        Me.PictureBox15 = New System.Windows.Forms.PictureBox()
        Me.PictureBox14 = New System.Windows.Forms.PictureBox()
        Me.PictureBox13 = New System.Windows.Forms.PictureBox()
        Me.PictureBox12 = New System.Windows.Forms.PictureBox()
        Me.PictureBox11 = New System.Windows.Forms.PictureBox()
        Me.PictureBox10 = New System.Windows.Forms.PictureBox()
        Me.PictureBox9 = New System.Windows.Forms.PictureBox()
        Me.PictureBox8 = New System.Windows.Forms.PictureBox()
        Me.PictureBox7 = New System.Windows.Forms.PictureBox()
        Me.PictureBox6 = New System.Windows.Forms.PictureBox()
        Me.PictureBox5 = New System.Windows.Forms.PictureBox()
        Me.PictureBox4 = New System.Windows.Forms.PictureBox()
        Me.PictureBox3 = New System.Windows.Forms.PictureBox()
        Me.PictureBox2 = New System.Windows.Forms.PictureBox()
        Me.PictureBox1 = New System.Windows.Forms.PictureBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.profile = New System.Windows.Forms.PictureBox()
        Me.logo = New System.Windows.Forms.PictureBox()
        Me.РадиоToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.PaintToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem1 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem2 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem4 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem5 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem6 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem7 = New System.Windows.Forms.ToolStripMenuItem()
        Me.Check_ls = New System.Windows.Forms.ToolStripMenuItem()
        Me.ICQToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ФайлToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.РедактироватьПрофильToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripSeparator1 = New System.Windows.Forms.ToolStripSeparator()
        Me.ВыходToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ШрифтыToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ВыбратьШрифтToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ВыбратьЦветToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ОчитскаСообщенийToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ЛичныеСобщенияToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem3 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem8 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem9 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem10 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem11 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem12 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem13 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem15 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem16 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem17 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem19 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem21 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem22 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem26 = New System.Windows.Forms.ToolStripMenuItem()
        Me.ToolStripMenuItem27 = New System.Windows.Forms.ToolStripMenuItem()
        Me.GB_Chat.SuspendLayout()
        Me.GB_Smails.SuspendLayout()
        Me.GB_Users.SuspendLayout()
        Me.GB_XZ2.SuspendLayout()
        Me.GB_Xz.SuspendLayout()
        Me.MenuStrip1.SuspendLayout()
        CType(Me.pctr_prikrep, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.sendmes, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Avatar, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox16, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox15, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox14, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox13, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox12, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox11, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox10, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox9, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox8, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox7, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox6, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox5, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox4, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox3, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox2, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PictureBox1, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.profile, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.logo, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'BWorker_chat
        '
        Me.BWorker_chat.WorkerSupportsCancellation = True
        '
        'tmr_chat
        '
        Me.tmr_chat.Enabled = True
        Me.tmr_chat.Interval = 1000
        '
        'GB_Chat
        '
        Me.GB_Chat.Controls.Add(Me.GB_Smails)
        Me.GB_Chat.Controls.Add(Me.Rtxt_chat)
        Me.GB_Chat.Controls.Add(Me.ChatBoxBackup)
        Me.GB_Chat.Location = New System.Drawing.Point(14, 72)
        Me.GB_Chat.Name = "GB_Chat"
        Me.GB_Chat.Size = New System.Drawing.Size(437, 275)
        Me.GB_Chat.TabIndex = 0
        Me.GB_Chat.TabStop = False
        '
        'GB_Smails
        '
        Me.GB_Smails.BackColor = System.Drawing.Color.Black
        Me.GB_Smails.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None
        Me.GB_Smails.Controls.Add(Me.PictureBox16)
        Me.GB_Smails.Controls.Add(Me.PictureBox15)
        Me.GB_Smails.Controls.Add(Me.PictureBox14)
        Me.GB_Smails.Controls.Add(Me.PictureBox13)
        Me.GB_Smails.Controls.Add(Me.PictureBox12)
        Me.GB_Smails.Controls.Add(Me.PictureBox11)
        Me.GB_Smails.Controls.Add(Me.PictureBox10)
        Me.GB_Smails.Controls.Add(Me.PictureBox9)
        Me.GB_Smails.Controls.Add(Me.PictureBox8)
        Me.GB_Smails.Controls.Add(Me.PictureBox7)
        Me.GB_Smails.Controls.Add(Me.PictureBox6)
        Me.GB_Smails.Controls.Add(Me.PictureBox5)
        Me.GB_Smails.Controls.Add(Me.PictureBox4)
        Me.GB_Smails.Controls.Add(Me.PictureBox3)
        Me.GB_Smails.Controls.Add(Me.PictureBox2)
        Me.GB_Smails.Controls.Add(Me.PictureBox1)
        Me.GB_Smails.Cursor = System.Windows.Forms.Cursors.Hand
        Me.GB_Smails.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.GB_Smails.ForeColor = System.Drawing.Color.White
        Me.GB_Smails.Location = New System.Drawing.Point(41, 15)
        Me.GB_Smails.Name = "GB_Smails"
        Me.GB_Smails.Size = New System.Drawing.Size(357, 96)
        Me.GB_Smails.TabIndex = 2
        Me.GB_Smails.TabStop = False
        '
        'Rtxt_chat
        '
        Me.Rtxt_chat.BackColor = System.Drawing.Color.Black
        Me.Rtxt_chat.BorderStyle = System.Windows.Forms.BorderStyle.None
        Me.Rtxt_chat.Cursor = System.Windows.Forms.Cursors.Default
        Me.Rtxt_chat.Font = New System.Drawing.Font("Comic Sans MS", 9.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Rtxt_chat.ForeColor = System.Drawing.Color.White
        Me.Rtxt_chat.Location = New System.Drawing.Point(7, 11)
        Me.Rtxt_chat.MaxLength = 32000
        Me.Rtxt_chat.Name = "Rtxt_chat"
        Me.Rtxt_chat.Size = New System.Drawing.Size(424, 258)
        Me.Rtxt_chat.TabIndex = 0
        Me.Rtxt_chat.Text = ""
        '
        'ChatBoxBackup
        '
        Me.ChatBoxBackup.Location = New System.Drawing.Point(7, 11)
        Me.ChatBoxBackup.Name = "ChatBoxBackup"
        Me.ChatBoxBackup.Size = New System.Drawing.Size(22, 19)
        Me.ChatBoxBackup.TabIndex = 1
        Me.ChatBoxBackup.Text = ""
        '
        'GB_Users
        '
        Me.GB_Users.Controls.Add(Me.lst_friends)
        Me.GB_Users.Controls.Add(Me.GB_XZ2)
        Me.GB_Users.Controls.Add(Me.Avatar)
        Me.GB_Users.Location = New System.Drawing.Point(498, 72)
        Me.GB_Users.Name = "GB_Users"
        Me.GB_Users.Size = New System.Drawing.Size(285, 275)
        Me.GB_Users.TabIndex = 1
        Me.GB_Users.TabStop = False
        '
        'lst_friends
        '
        Me.lst_friends.BackColor = System.Drawing.Color.Black
        Me.lst_friends.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.lst_friends.ForeColor = System.Drawing.Color.White
        Me.lst_friends.FormattingEnabled = True
        Me.lst_friends.Location = New System.Drawing.Point(9, 145)
        Me.lst_friends.Name = "lst_friends"
        Me.lst_friends.Size = New System.Drawing.Size(268, 119)
        Me.lst_friends.TabIndex = 0
        '
        'GB_XZ2
        '
        Me.GB_XZ2.BackColor = System.Drawing.Color.Black
        Me.GB_XZ2.Controls.Add(Me.phonework)
        Me.GB_XZ2.Controls.Add(Me.mailes)
        Me.GB_XZ2.Controls.Add(Me.Username)
        Me.GB_XZ2.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.GB_XZ2.Location = New System.Drawing.Point(132, 8)
        Me.GB_XZ2.Name = "GB_XZ2"
        Me.GB_XZ2.Size = New System.Drawing.Size(146, 127)
        Me.GB_XZ2.TabIndex = 2
        Me.GB_XZ2.TabStop = False
        '
        'phonework
        '
        Me.phonework.AutoSize = True
        Me.phonework.Font = New System.Drawing.Font("Comic Sans MS", 9.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.phonework.ForeColor = System.Drawing.Color.White
        Me.phonework.Location = New System.Drawing.Point(6, 78)
        Me.phonework.Name = "phonework"
        Me.phonework.Size = New System.Drawing.Size(115, 18)
        Me.phonework.TabIndex = 0
        Me.phonework.Text = "Номер телефона"
        '
        'mailes
        '
        Me.mailes.AutoSize = True
        Me.mailes.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.mailes.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.mailes.ForeColor = System.Drawing.Color.White
        Me.mailes.Location = New System.Drawing.Point(6, 101)
        Me.mailes.Name = "mailes"
        Me.mailes.Size = New System.Drawing.Size(59, 23)
        Me.mailes.TabIndex = 0
        Me.mailes.Text = "Почта"
        '
        'Username
        '
        Me.Username.AutoSize = True
        Me.Username.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Username.ForeColor = System.Drawing.Color.White
        Me.Username.Location = New System.Drawing.Point(6, 11)
        Me.Username.Name = "Username"
        Me.Username.Size = New System.Drawing.Size(79, 23)
        Me.Username.TabIndex = 0
        Me.Username.Text = "Никнейм"
        '
        'GB_Xz
        '
        Me.GB_Xz.Controls.Add(Me.pctr_prikrep)
        Me.GB_Xz.Controls.Add(Me.sendmes)
        Me.GB_Xz.Controls.Add(Me.history)
        Me.GB_Xz.Controls.Add(Me.smile_fon)
        Me.GB_Xz.Location = New System.Drawing.Point(14, 353)
        Me.GB_Xz.Name = "GB_Xz"
        Me.GB_Xz.Size = New System.Drawing.Size(437, 90)
        Me.GB_Xz.TabIndex = 1
        Me.GB_Xz.TabStop = False
        '
        'history
        '
        Me.history.BackColor = System.Drawing.Color.Black
        Me.history.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.history.ForeColor = System.Drawing.Color.White
        Me.history.Location = New System.Drawing.Point(41, 11)
        Me.history.MaxLength = 200
        Me.history.Multiline = True
        Me.history.Name = "history"
        Me.history.ScrollBars = System.Windows.Forms.ScrollBars.Vertical
        Me.history.Size = New System.Drawing.Size(277, 72)
        Me.history.TabIndex = 0
        '
        'OpenFileDialog2
        '
        Me.OpenFileDialog2.FileName = "OpenFileDialog1"
        '
        'OpenFileDialog1
        '
        Me.OpenFileDialog1.FileName = "OpenFileDialog1"
        '
        'btn_naps
        '
        Me.btn_naps.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.btn_naps.Location = New System.Drawing.Point(774, 442)
        Me.btn_naps.Name = "btn_naps"
        Me.btn_naps.Size = New System.Drawing.Size(1, 1)
        Me.btn_naps.TabIndex = 12
        Me.btn_naps.UseVisualStyleBackColor = True
        '
        'РазвлеченияToolStripMenuItem
        '
        Me.РазвлеченияToolStripMenuItem.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.РадиоToolStripMenuItem, Me.PaintToolStripMenuItem})
        Me.РазвлеченияToolStripMenuItem.ForeColor = System.Drawing.Color.White
        Me.РазвлеченияToolStripMenuItem.Name = "РазвлеченияToolStripMenuItem"
        Me.РазвлеченияToolStripMenuItem.Size = New System.Drawing.Size(98, 23)
        Me.РазвлеченияToolStripMenuItem.Text = "Развлечения"
        '
        'txtBoxHelper
        '
        Me.txtBoxHelper.Location = New System.Drawing.Point(792, 444)
        Me.txtBoxHelper.Multiline = True
        Me.txtBoxHelper.Name = "txtBoxHelper"
        Me.txtBoxHelper.Size = New System.Drawing.Size(1, 1)
        Me.txtBoxHelper.TabIndex = 0
        Me.txtBoxHelper.Visible = False
        '
        'ToolStripMenuItem14
        '
        Me.ToolStripMenuItem14.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem15, Me.ToolStripMenuItem16})
        Me.ToolStripMenuItem14.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem14.Name = "ToolStripMenuItem14"
        Me.ToolStripMenuItem14.Size = New System.Drawing.Size(98, 23)
        Me.ToolStripMenuItem14.Text = "Развлечения"
        '
        'MenuStrip1
        '
        Me.MenuStrip1.AutoSize = False
        Me.MenuStrip1.BackgroundImage = Global.TSchat.My.Resources.Resources.black
        Me.MenuStrip1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None
        Me.MenuStrip1.Dock = System.Windows.Forms.DockStyle.None
        Me.MenuStrip1.Font = New System.Drawing.Font("Comic Sans MS", 9.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.MenuStrip1.Items.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem18, Me.ToolStripMenuItem20, Me.ToolStripMenuItem23, Me.ToolStripMenuItem24, Me.ToolStripMenuItem25, Me.ToolStripMenuItem28})
        Me.MenuStrip1.Location = New System.Drawing.Point(0, 38)
        Me.MenuStrip1.Name = "MenuStrip1"
        Me.MenuStrip1.Size = New System.Drawing.Size(799, 27)
        Me.MenuStrip1.TabIndex = 14
        '
        'ToolStripMenuItem18
        '
        Me.ToolStripMenuItem18.BackColor = System.Drawing.Color.Black
        Me.ToolStripMenuItem18.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.РедактироватьПрофильToolStripMenuItem1})
        Me.ToolStripMenuItem18.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem18.Image = Global.TSchat.My.Resources.Resources.Fail
        Me.ToolStripMenuItem18.ImageTransparentColor = System.Drawing.Color.White
        Me.ToolStripMenuItem18.Name = "ToolStripMenuItem18"
        Me.ToolStripMenuItem18.Size = New System.Drawing.Size(67, 23)
        Me.ToolStripMenuItem18.Text = "Файл"
        '
        'РедактироватьПрофильToolStripMenuItem1
        '
        Me.РедактироватьПрофильToolStripMenuItem1.Image = Global.TSchat.My.Resources.Resources.redakt
        Me.РедактироватьПрофильToolStripMenuItem1.Name = "РедактироватьПрофильToolStripMenuItem1"
        Me.РедактироватьПрофильToolStripMenuItem1.Size = New System.Drawing.Size(226, 22)
        Me.РедактироватьПрофильToolStripMenuItem1.Text = "Редактировать профиль"
        '
        'ToolStripMenuItem20
        '
        Me.ToolStripMenuItem20.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ВыбратьШрифтToolStripMenuItem1, Me.ВыбратьЦветToolStripMenuItem1})
        Me.ToolStripMenuItem20.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem20.Image = Global.TSchat.My.Resources.Resources.fontes
        Me.ToolStripMenuItem20.Name = "ToolStripMenuItem20"
        Me.ToolStripMenuItem20.Size = New System.Drawing.Size(90, 23)
        Me.ToolStripMenuItem20.Text = "Шрифты"
        '
        'ВыбратьШрифтToolStripMenuItem1
        '
        Me.ВыбратьШрифтToolStripMenuItem1.Image = Global.TSchat.My.Resources.Resources.fontB
        Me.ВыбратьШрифтToolStripMenuItem1.Name = "ВыбратьШрифтToolStripMenuItem1"
        Me.ВыбратьШрифтToolStripMenuItem1.Size = New System.Drawing.Size(173, 22)
        Me.ВыбратьШрифтToolStripMenuItem1.Text = "Выбрать шрифт"
        '
        'ВыбратьЦветToolStripMenuItem1
        '
        Me.ВыбратьЦветToolStripMenuItem1.Image = Global.TSchat.My.Resources.Resources.rainbow
        Me.ВыбратьЦветToolStripMenuItem1.Name = "ВыбратьЦветToolStripMenuItem1"
        Me.ВыбратьЦветToolStripMenuItem1.Size = New System.Drawing.Size(173, 22)
        Me.ВыбратьЦветToolStripMenuItem1.Text = "Выбрать цвет"
        '
        'ToolStripMenuItem23
        '
        Me.ToolStripMenuItem23.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem23.Image = Global.TSchat.My.Resources.Resources.clear
        Me.ToolStripMenuItem23.Name = "ToolStripMenuItem23"
        Me.ToolStripMenuItem23.Size = New System.Drawing.Size(161, 23)
        Me.ToolStripMenuItem23.Text = "Очистка сообщений"
        '
        'ToolStripMenuItem24
        '
        Me.ToolStripMenuItem24.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ToolStripMenuItem24.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem24.Image = Global.TSchat.My.Resources.Resources.lmseg
        Me.ToolStripMenuItem24.Name = "ToolStripMenuItem24"
        Me.ToolStripMenuItem24.Size = New System.Drawing.Size(150, 23)
        Me.ToolStripMenuItem24.Text = "Личные собщения"
        '
        'ToolStripMenuItem25
        '
        Me.ToolStripMenuItem25.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.РадиоToolStripMenuItem1, Me.ЗаметкиToolStripMenuItem})
        Me.ToolStripMenuItem25.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem25.Name = "ToolStripMenuItem25"
        Me.ToolStripMenuItem25.Size = New System.Drawing.Size(98, 23)
        Me.ToolStripMenuItem25.Text = "Развлечения"
        '
        'РадиоToolStripMenuItem1
        '
        Me.РадиоToolStripMenuItem1.Image = Global.TSchat.My.Resources.Resources.radio
        Me.РадиоToolStripMenuItem1.Name = "РадиоToolStripMenuItem1"
        Me.РадиоToolStripMenuItem1.Size = New System.Drawing.Size(180, 22)
        Me.РадиоToolStripMenuItem1.Text = "Радио"
        '
        'ЗаметкиToolStripMenuItem
        '
        Me.ЗаметкиToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.pencil
        Me.ЗаметкиToolStripMenuItem.Name = "ЗаметкиToolStripMenuItem"
        Me.ЗаметкиToolStripMenuItem.Size = New System.Drawing.Size(180, 22)
        Me.ЗаметкиToolStripMenuItem.Text = "Заметки"
        '
        'ToolStripMenuItem28
        '
        Me.ToolStripMenuItem28.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ToolStripMenuItem28.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem28.Image = Global.TSchat.My.Resources.Resources.icq_lgtp
        Me.ToolStripMenuItem28.Name = "ToolStripMenuItem28"
        Me.ToolStripMenuItem28.Size = New System.Drawing.Size(62, 23)
        Me.ToolStripMenuItem28.Text = "ICQ"
        '
        'pctr_prikrep
        '
        Me.pctr_prikrep.Cursor = System.Windows.Forms.Cursors.Hand
        Me.pctr_prikrep.Image = Global.TSchat.My.Resources.Resources.prikrep
        Me.pctr_prikrep.Location = New System.Drawing.Point(382, 21)
        Me.pctr_prikrep.Name = "pctr_prikrep"
        Me.pctr_prikrep.Size = New System.Drawing.Size(45, 45)
        Me.pctr_prikrep.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.pctr_prikrep.TabIndex = 4
        Me.pctr_prikrep.TabStop = False
        Me.Hint_User.SetToolTip(Me.pctr_prikrep, "Выбрать файл для отправки")
        '
        'sendmes
        '
        Me.sendmes.Cursor = System.Windows.Forms.Cursors.Hand
        Me.sendmes.Image = Global.TSchat.My.Resources.Resources.Send_mail
        Me.sendmes.Location = New System.Drawing.Point(324, 19)
        Me.sendmes.Name = "sendmes"
        Me.sendmes.Size = New System.Drawing.Size(50, 50)
        Me.sendmes.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.sendmes.TabIndex = 3
        Me.sendmes.TabStop = False
        '
        'smile_fon
        '
        Me.smile_fon.BackgroundImage = CType(resources.GetObject("smile_fon.BackgroundImage"), System.Drawing.Image)
        Me.smile_fon.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch
        Me.smile_fon.Cursor = System.Windows.Forms.Cursors.Hand
        Me.smile_fon.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.smile_fon.Location = New System.Drawing.Point(3, 47)
        Me.smile_fon.Name = "smile_fon"
        Me.smile_fon.Size = New System.Drawing.Size(35, 35)
        Me.smile_fon.TabIndex = 1
        Me.smile_fon.UseVisualStyleBackColor = True
        '
        'Avatar
        '
        Me.Avatar.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.Avatar.Cursor = System.Windows.Forms.Cursors.Hand
        Me.Avatar.Location = New System.Drawing.Point(6, 15)
        Me.Avatar.Name = "Avatar"
        Me.Avatar.Size = New System.Drawing.Size(120, 120)
        Me.Avatar.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.Avatar.TabIndex = 0
        Me.Avatar.TabStop = False
        Me.Hint_User.SetToolTip(Me.Avatar, "Выбор фотографии профиля")
        '
        'PictureBox16
        '
        Me.PictureBox16.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox16.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox16.Image = Global.TSchat.My.Resources.Resources._16
        Me.PictureBox16.Location = New System.Drawing.Point(311, 51)
        Me.PictureBox16.Name = "PictureBox16"
        Me.PictureBox16.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox16.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox16.TabIndex = 20
        Me.PictureBox16.TabStop = False
        '
        'PictureBox15
        '
        Me.PictureBox15.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox15.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox15.Image = Global.TSchat.My.Resources.Resources._15
        Me.PictureBox15.Location = New System.Drawing.Point(266, 51)
        Me.PictureBox15.Name = "PictureBox15"
        Me.PictureBox15.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox15.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox15.TabIndex = 19
        Me.PictureBox15.TabStop = False
        '
        'PictureBox14
        '
        Me.PictureBox14.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox14.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox14.Image = Global.TSchat.My.Resources.Resources._14
        Me.PictureBox14.Location = New System.Drawing.Point(222, 51)
        Me.PictureBox14.Name = "PictureBox14"
        Me.PictureBox14.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox14.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox14.TabIndex = 18
        Me.PictureBox14.TabStop = False
        '
        'PictureBox13
        '
        Me.PictureBox13.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox13.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox13.Image = Global.TSchat.My.Resources.Resources._13
        Me.PictureBox13.Location = New System.Drawing.Point(178, 51)
        Me.PictureBox13.Name = "PictureBox13"
        Me.PictureBox13.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox13.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox13.TabIndex = 17
        Me.PictureBox13.TabStop = False
        '
        'PictureBox12
        '
        Me.PictureBox12.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox12.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox12.Image = Global.TSchat.My.Resources.Resources._12
        Me.PictureBox12.Location = New System.Drawing.Point(134, 51)
        Me.PictureBox12.Name = "PictureBox12"
        Me.PictureBox12.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox12.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox12.TabIndex = 16
        Me.PictureBox12.TabStop = False
        '
        'PictureBox11
        '
        Me.PictureBox11.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox11.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox11.Image = Global.TSchat.My.Resources.Resources._11
        Me.PictureBox11.Location = New System.Drawing.Point(90, 51)
        Me.PictureBox11.Name = "PictureBox11"
        Me.PictureBox11.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox11.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox11.TabIndex = 15
        Me.PictureBox11.TabStop = False
        '
        'PictureBox10
        '
        Me.PictureBox10.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox10.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox10.Image = Global.TSchat.My.Resources.Resources._10
        Me.PictureBox10.Location = New System.Drawing.Point(46, 51)
        Me.PictureBox10.Name = "PictureBox10"
        Me.PictureBox10.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox10.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox10.TabIndex = 14
        Me.PictureBox10.TabStop = False
        '
        'PictureBox9
        '
        Me.PictureBox9.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox9.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox9.Image = Global.TSchat.My.Resources.Resources._9
        Me.PictureBox9.Location = New System.Drawing.Point(2, 51)
        Me.PictureBox9.Name = "PictureBox9"
        Me.PictureBox9.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox9.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox9.TabIndex = 13
        Me.PictureBox9.TabStop = False
        '
        'PictureBox8
        '
        Me.PictureBox8.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox8.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox8.Image = Global.TSchat.My.Resources.Resources._8
        Me.PictureBox8.Location = New System.Drawing.Point(311, 9)
        Me.PictureBox8.Name = "PictureBox8"
        Me.PictureBox8.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox8.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox8.TabIndex = 12
        Me.PictureBox8.TabStop = False
        '
        'PictureBox7
        '
        Me.PictureBox7.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox7.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox7.Image = Global.TSchat.My.Resources.Resources._7
        Me.PictureBox7.Location = New System.Drawing.Point(266, 9)
        Me.PictureBox7.Name = "PictureBox7"
        Me.PictureBox7.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox7.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox7.TabIndex = 11
        Me.PictureBox7.TabStop = False
        '
        'PictureBox6
        '
        Me.PictureBox6.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox6.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox6.Image = Global.TSchat.My.Resources.Resources._6
        Me.PictureBox6.Location = New System.Drawing.Point(222, 9)
        Me.PictureBox6.Name = "PictureBox6"
        Me.PictureBox6.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox6.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox6.TabIndex = 10
        Me.PictureBox6.TabStop = False
        '
        'PictureBox5
        '
        Me.PictureBox5.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox5.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox5.Image = Global.TSchat.My.Resources.Resources._5
        Me.PictureBox5.Location = New System.Drawing.Point(178, 9)
        Me.PictureBox5.Name = "PictureBox5"
        Me.PictureBox5.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox5.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox5.TabIndex = 9
        Me.PictureBox5.TabStop = False
        '
        'PictureBox4
        '
        Me.PictureBox4.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox4.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox4.Image = Global.TSchat.My.Resources.Resources._4
        Me.PictureBox4.Location = New System.Drawing.Point(134, 9)
        Me.PictureBox4.Name = "PictureBox4"
        Me.PictureBox4.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox4.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox4.TabIndex = 8
        Me.PictureBox4.TabStop = False
        '
        'PictureBox3
        '
        Me.PictureBox3.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox3.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox3.Image = Global.TSchat.My.Resources.Resources._3
        Me.PictureBox3.Location = New System.Drawing.Point(90, 9)
        Me.PictureBox3.Name = "PictureBox3"
        Me.PictureBox3.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox3.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox3.TabIndex = 7
        Me.PictureBox3.TabStop = False
        '
        'PictureBox2
        '
        Me.PictureBox2.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox2.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox2.Image = Global.TSchat.My.Resources.Resources._2
        Me.PictureBox2.Location = New System.Drawing.Point(46, 9)
        Me.PictureBox2.Name = "PictureBox2"
        Me.PictureBox2.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox2.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox2.TabIndex = 6
        Me.PictureBox2.TabStop = False
        '
        'PictureBox1
        '
        Me.PictureBox1.BackColor = System.Drawing.Color.Transparent
        Me.PictureBox1.Cursor = System.Windows.Forms.Cursors.Hand
        Me.PictureBox1.Image = Global.TSchat.My.Resources.Resources._1
        Me.PictureBox1.Location = New System.Drawing.Point(2, 9)
        Me.PictureBox1.Name = "PictureBox1"
        Me.PictureBox1.Size = New System.Drawing.Size(42, 42)
        Me.PictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.PictureBox1.TabIndex = 5
        Me.PictureBox1.TabStop = False
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(716, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 8
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(757, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 7
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(2, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 6
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-4, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(803, 35)
        Me.Stringer.TabIndex = 5
        Me.Stringer.TabStop = False
        '
        'profile
        '
        Me.profile.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.profile.Cursor = System.Windows.Forms.Cursors.Hand
        Me.profile.Image = Global.TSchat.My.Resources.Resources.profile
        Me.profile.Location = New System.Drawing.Point(466, 91)
        Me.profile.Name = "profile"
        Me.profile.Size = New System.Drawing.Size(32, 243)
        Me.profile.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.profile.TabIndex = 10
        Me.profile.TabStop = False
        '
        'logo
        '
        Me.logo.Image = Global.TSchat.My.Resources.Resources._32323
        Me.logo.Location = New System.Drawing.Point(533, 322)
        Me.logo.Name = "logo"
        Me.logo.Size = New System.Drawing.Size(218, 140)
        Me.logo.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.logo.TabIndex = 9
        Me.logo.TabStop = False
        '
        'РадиоToolStripMenuItem
        '
        Me.РадиоToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.radio
        Me.РадиоToolStripMenuItem.Name = "РадиоToolStripMenuItem"
        Me.РадиоToolStripMenuItem.Size = New System.Drawing.Size(120, 22)
        Me.РадиоToolStripMenuItem.Text = "Радио"
        '
        'PaintToolStripMenuItem
        '
        Me.PaintToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.pencil
        Me.PaintToolStripMenuItem.Name = "PaintToolStripMenuItem"
        Me.PaintToolStripMenuItem.Size = New System.Drawing.Size(120, 22)
        Me.PaintToolStripMenuItem.Text = "Заметки"
        '
        'ToolStripMenuItem1
        '
        Me.ToolStripMenuItem1.BackColor = System.Drawing.Color.Black
        Me.ToolStripMenuItem1.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem2})
        Me.ToolStripMenuItem1.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem1.Image = Global.TSchat.My.Resources.Resources.Fail
        Me.ToolStripMenuItem1.Name = "ToolStripMenuItem1"
        Me.ToolStripMenuItem1.Size = New System.Drawing.Size(67, 23)
        Me.ToolStripMenuItem1.Text = "Файл"
        '
        'ToolStripMenuItem2
        '
        Me.ToolStripMenuItem2.Image = Global.TSchat.My.Resources.Resources.redakt
        Me.ToolStripMenuItem2.Name = "ToolStripMenuItem2"
        Me.ToolStripMenuItem2.Size = New System.Drawing.Size(207, 22)
        Me.ToolStripMenuItem2.Text = "Редактировать профиль"
        '
        'ToolStripMenuItem4
        '
        Me.ToolStripMenuItem4.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem5, Me.ToolStripMenuItem6})
        Me.ToolStripMenuItem4.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem4.Image = Global.TSchat.My.Resources.Resources.fontes
        Me.ToolStripMenuItem4.Name = "ToolStripMenuItem4"
        Me.ToolStripMenuItem4.Size = New System.Drawing.Size(90, 23)
        Me.ToolStripMenuItem4.Text = "Шрифты"
        '
        'ToolStripMenuItem5
        '
        Me.ToolStripMenuItem5.Image = Global.TSchat.My.Resources.Resources.fontB
        Me.ToolStripMenuItem5.Name = "ToolStripMenuItem5"
        Me.ToolStripMenuItem5.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem5.Text = "Выбрать шрифт"
        '
        'ToolStripMenuItem6
        '
        Me.ToolStripMenuItem6.Image = Global.TSchat.My.Resources.Resources.rainbow
        Me.ToolStripMenuItem6.Name = "ToolStripMenuItem6"
        Me.ToolStripMenuItem6.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem6.Text = "Выбрать цвет"
        '
        'ToolStripMenuItem7
        '
        Me.ToolStripMenuItem7.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem7.Image = Global.TSchat.My.Resources.Resources.clear
        Me.ToolStripMenuItem7.Name = "ToolStripMenuItem7"
        Me.ToolStripMenuItem7.Size = New System.Drawing.Size(161, 23)
        Me.ToolStripMenuItem7.Text = "Очистка сообщений"
        '
        'Check_ls
        '
        Me.Check_ls.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.Check_ls.ForeColor = System.Drawing.Color.White
        Me.Check_ls.Image = Global.TSchat.My.Resources.Resources.lmseg
        Me.Check_ls.Name = "Check_ls"
        Me.Check_ls.Size = New System.Drawing.Size(150, 23)
        Me.Check_ls.Text = "Личные собщения"
        '
        'ICQToolStripMenuItem
        '
        Me.ICQToolStripMenuItem.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ICQToolStripMenuItem.ForeColor = System.Drawing.Color.White
        Me.ICQToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.icq_lgtp
        Me.ICQToolStripMenuItem.Name = "ICQToolStripMenuItem"
        Me.ICQToolStripMenuItem.Size = New System.Drawing.Size(62, 23)
        Me.ICQToolStripMenuItem.Text = "ICQ"
        '
        'ФайлToolStripMenuItem
        '
        Me.ФайлToolStripMenuItem.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.РедактироватьПрофильToolStripMenuItem, Me.ToolStripSeparator1, Me.ВыходToolStripMenuItem})
        Me.ФайлToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.Fail
        Me.ФайлToolStripMenuItem.Name = "ФайлToolStripMenuItem"
        Me.ФайлToolStripMenuItem.Size = New System.Drawing.Size(67, 23)
        Me.ФайлToolStripMenuItem.Text = "Файл"
        '
        'РедактироватьПрофильToolStripMenuItem
        '
        Me.РедактироватьПрофильToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.redakt
        Me.РедактироватьПрофильToolStripMenuItem.Name = "РедактироватьПрофильToolStripMenuItem"
        Me.РедактироватьПрофильToolStripMenuItem.Size = New System.Drawing.Size(207, 22)
        Me.РедактироватьПрофильToolStripMenuItem.Text = "Редактировать профиль"
        '
        'ToolStripSeparator1
        '
        Me.ToolStripSeparator1.Name = "ToolStripSeparator1"
        Me.ToolStripSeparator1.Size = New System.Drawing.Size(204, 6)
        '
        'ВыходToolStripMenuItem
        '
        Me.ВыходToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.exiter
        Me.ВыходToolStripMenuItem.Name = "ВыходToolStripMenuItem"
        Me.ВыходToolStripMenuItem.Size = New System.Drawing.Size(207, 22)
        Me.ВыходToolStripMenuItem.Text = "Выход"
        '
        'ШрифтыToolStripMenuItem
        '
        Me.ШрифтыToolStripMenuItem.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ВыбратьШрифтToolStripMenuItem, Me.ВыбратьЦветToolStripMenuItem})
        Me.ШрифтыToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.fontes
        Me.ШрифтыToolStripMenuItem.Name = "ШрифтыToolStripMenuItem"
        Me.ШрифтыToolStripMenuItem.Size = New System.Drawing.Size(90, 23)
        Me.ШрифтыToolStripMenuItem.Text = "Шрифты"
        '
        'ВыбратьШрифтToolStripMenuItem
        '
        Me.ВыбратьШрифтToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.fontB
        Me.ВыбратьШрифтToolStripMenuItem.Name = "ВыбратьШрифтToolStripMenuItem"
        Me.ВыбратьШрифтToolStripMenuItem.Size = New System.Drawing.Size(163, 22)
        Me.ВыбратьШрифтToolStripMenuItem.Text = "Выбрать шрифт"
        '
        'ВыбратьЦветToolStripMenuItem
        '
        Me.ВыбратьЦветToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.rainbow
        Me.ВыбратьЦветToolStripMenuItem.Name = "ВыбратьЦветToolStripMenuItem"
        Me.ВыбратьЦветToolStripMenuItem.Size = New System.Drawing.Size(163, 22)
        Me.ВыбратьЦветToolStripMenuItem.Text = "Выбрать цвет"
        '
        'ОчитскаСообщенийToolStripMenuItem
        '
        Me.ОчитскаСообщенийToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.clear
        Me.ОчитскаСообщенийToolStripMenuItem.Name = "ОчитскаСообщенийToolStripMenuItem"
        Me.ОчитскаСообщенийToolStripMenuItem.Size = New System.Drawing.Size(161, 23)
        Me.ОчитскаСообщенийToolStripMenuItem.Text = "Очистка сообщений"
        '
        'ЛичныеСобщенияToolStripMenuItem
        '
        Me.ЛичныеСобщенияToolStripMenuItem.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ЛичныеСобщенияToolStripMenuItem.Image = Global.TSchat.My.Resources.Resources.lmseg
        Me.ЛичныеСобщенияToolStripMenuItem.Name = "ЛичныеСобщенияToolStripMenuItem"
        Me.ЛичныеСобщенияToolStripMenuItem.Size = New System.Drawing.Size(150, 23)
        Me.ЛичныеСобщенияToolStripMenuItem.Text = "Личные собщения"
        '
        'ToolStripMenuItem3
        '
        Me.ToolStripMenuItem3.BackColor = System.Drawing.Color.Black
        Me.ToolStripMenuItem3.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem8})
        Me.ToolStripMenuItem3.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem3.Image = Global.TSchat.My.Resources.Resources.Fail
        Me.ToolStripMenuItem3.Name = "ToolStripMenuItem3"
        Me.ToolStripMenuItem3.Size = New System.Drawing.Size(67, 23)
        Me.ToolStripMenuItem3.Text = "Файл"
        '
        'ToolStripMenuItem8
        '
        Me.ToolStripMenuItem8.Image = Global.TSchat.My.Resources.Resources.redakt
        Me.ToolStripMenuItem8.Name = "ToolStripMenuItem8"
        Me.ToolStripMenuItem8.Size = New System.Drawing.Size(207, 22)
        Me.ToolStripMenuItem8.Text = "Редактировать профиль"
        '
        'ToolStripMenuItem9
        '
        Me.ToolStripMenuItem9.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ToolStripMenuItem10, Me.ToolStripMenuItem11})
        Me.ToolStripMenuItem9.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem9.Image = Global.TSchat.My.Resources.Resources.fontes
        Me.ToolStripMenuItem9.Name = "ToolStripMenuItem9"
        Me.ToolStripMenuItem9.Size = New System.Drawing.Size(90, 23)
        Me.ToolStripMenuItem9.Text = "Шрифты"
        '
        'ToolStripMenuItem10
        '
        Me.ToolStripMenuItem10.Image = Global.TSchat.My.Resources.Resources.fontB
        Me.ToolStripMenuItem10.Name = "ToolStripMenuItem10"
        Me.ToolStripMenuItem10.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem10.Text = "Выбрать шрифт"
        '
        'ToolStripMenuItem11
        '
        Me.ToolStripMenuItem11.Image = Global.TSchat.My.Resources.Resources.rainbow
        Me.ToolStripMenuItem11.Name = "ToolStripMenuItem11"
        Me.ToolStripMenuItem11.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem11.Text = "Выбрать цвет"
        '
        'ToolStripMenuItem12
        '
        Me.ToolStripMenuItem12.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem12.Image = Global.TSchat.My.Resources.Resources.clear
        Me.ToolStripMenuItem12.Name = "ToolStripMenuItem12"
        Me.ToolStripMenuItem12.Size = New System.Drawing.Size(161, 23)
        Me.ToolStripMenuItem12.Text = "Очистка сообщений"
        '
        'ToolStripMenuItem13
        '
        Me.ToolStripMenuItem13.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ToolStripMenuItem13.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem13.Image = Global.TSchat.My.Resources.Resources.lmseg
        Me.ToolStripMenuItem13.Name = "ToolStripMenuItem13"
        Me.ToolStripMenuItem13.Size = New System.Drawing.Size(150, 23)
        Me.ToolStripMenuItem13.Text = "Личные собщения"
        '
        'ToolStripMenuItem15
        '
        Me.ToolStripMenuItem15.Image = Global.TSchat.My.Resources.Resources.radio
        Me.ToolStripMenuItem15.Name = "ToolStripMenuItem15"
        Me.ToolStripMenuItem15.Size = New System.Drawing.Size(120, 22)
        Me.ToolStripMenuItem15.Text = "Радио"
        '
        'ToolStripMenuItem16
        '
        Me.ToolStripMenuItem16.Image = Global.TSchat.My.Resources.Resources.pencil
        Me.ToolStripMenuItem16.Name = "ToolStripMenuItem16"
        Me.ToolStripMenuItem16.Size = New System.Drawing.Size(120, 22)
        Me.ToolStripMenuItem16.Text = "Заметки"
        '
        'ToolStripMenuItem17
        '
        Me.ToolStripMenuItem17.Alignment = System.Windows.Forms.ToolStripItemAlignment.Right
        Me.ToolStripMenuItem17.ForeColor = System.Drawing.Color.White
        Me.ToolStripMenuItem17.Image = Global.TSchat.My.Resources.Resources.icq_lgtp
        Me.ToolStripMenuItem17.Name = "ToolStripMenuItem17"
        Me.ToolStripMenuItem17.Size = New System.Drawing.Size(62, 23)
        Me.ToolStripMenuItem17.Text = "ICQ"
        '
        'ToolStripMenuItem19
        '
        Me.ToolStripMenuItem19.Image = Global.TSchat.My.Resources.Resources.redakt
        Me.ToolStripMenuItem19.Name = "ToolStripMenuItem19"
        Me.ToolStripMenuItem19.Size = New System.Drawing.Size(207, 22)
        Me.ToolStripMenuItem19.Text = "Редактировать профиль"
        '
        'ToolStripMenuItem21
        '
        Me.ToolStripMenuItem21.Image = Global.TSchat.My.Resources.Resources.fontB
        Me.ToolStripMenuItem21.Name = "ToolStripMenuItem21"
        Me.ToolStripMenuItem21.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem21.Text = "Выбрать шрифт"
        '
        'ToolStripMenuItem22
        '
        Me.ToolStripMenuItem22.Image = Global.TSchat.My.Resources.Resources.rainbow
        Me.ToolStripMenuItem22.Name = "ToolStripMenuItem22"
        Me.ToolStripMenuItem22.Size = New System.Drawing.Size(163, 22)
        Me.ToolStripMenuItem22.Text = "Выбрать цвет"
        '
        'ToolStripMenuItem26
        '
        Me.ToolStripMenuItem26.Image = Global.TSchat.My.Resources.Resources.radio
        Me.ToolStripMenuItem26.Name = "ToolStripMenuItem26"
        Me.ToolStripMenuItem26.Size = New System.Drawing.Size(180, 22)
        Me.ToolStripMenuItem26.Text = "Радио"
        '
        'ToolStripMenuItem27
        '
        Me.ToolStripMenuItem27.Image = Global.TSchat.My.Resources.Resources.pencil
        Me.ToolStripMenuItem27.Name = "ToolStripMenuItem27"
        Me.ToolStripMenuItem27.Size = New System.Drawing.Size(180, 22)
        Me.ToolStripMenuItem27.Text = "Заметки"
        '
        'frm_chat
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(796, 451)
        Me.Controls.Add(Me.MenuStrip1)
        Me.Controls.Add(Me.txtBoxHelper)
        Me.Controls.Add(Me.btn_naps)
        Me.Controls.Add(Me.GB_Xz)
        Me.Controls.Add(Me.GB_Users)
        Me.Controls.Add(Me.GB_Chat)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.Controls.Add(Me.profile)
        Me.Controls.Add(Me.logo)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "frm_chat"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "Чат"
        Me.GB_Chat.ResumeLayout(False)
        Me.GB_Smails.ResumeLayout(False)
        Me.GB_Users.ResumeLayout(False)
        Me.GB_XZ2.ResumeLayout(False)
        Me.GB_XZ2.PerformLayout()
        Me.GB_Xz.ResumeLayout(False)
        Me.GB_Xz.PerformLayout()
        Me.MenuStrip1.ResumeLayout(False)
        Me.MenuStrip1.PerformLayout()
        CType(Me.pctr_prikrep, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.sendmes, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Avatar, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox16, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox15, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox14, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox13, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox12, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox11, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox10, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox9, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox8, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox7, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox6, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox5, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox4, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox3, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox2, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PictureBox1, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.profile, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.logo, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents Stringer As System.Windows.Forms.PictureBox
    Friend WithEvents BWorker_chat As System.ComponentModel.BackgroundWorker
    Friend WithEvents Colors As System.Windows.Forms.ColorDialog
    Friend WithEvents FontS As System.Windows.Forms.FontDialog
    Friend WithEvents tmr_chat As System.Windows.Forms.Timer
    Friend WithEvents ФайлToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ШрифтыToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ОчитскаСообщенийToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ЛичныеСобщенияToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents РедактироватьПрофильToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ToolStripSeparator1 As System.Windows.Forms.ToolStripSeparator
    Friend WithEvents ВыходToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ВыбратьШрифтToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents ВыбратьЦветToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents GB_Chat As System.Windows.Forms.GroupBox
    Friend WithEvents GB_Users As System.Windows.Forms.GroupBox
    Friend WithEvents GB_Xz As System.Windows.Forms.GroupBox
    Friend WithEvents Rtxt_chat As System.Windows.Forms.RichTextBox
    Friend WithEvents Avatar As System.Windows.Forms.PictureBox
    Friend WithEvents GB_XZ2 As System.Windows.Forms.GroupBox
    Friend WithEvents logo As System.Windows.Forms.PictureBox
    Friend WithEvents profile As System.Windows.Forms.PictureBox
    Friend WithEvents history As System.Windows.Forms.TextBox
    Friend WithEvents lst_friends As System.Windows.Forms.ListBox
    Friend WithEvents sendmes As System.Windows.Forms.PictureBox
    Friend WithEvents Username As System.Windows.Forms.Label
    Friend WithEvents phonework As System.Windows.Forms.Label
    Friend WithEvents mailes As System.Windows.Forms.Label
    Friend WithEvents ChatBoxBackup As System.Windows.Forms.RichTextBox
    Friend WithEvents pctr_prikrep As PictureBox
    Friend WithEvents OpenFileDialog2 As OpenFileDialog
    Friend WithEvents OpenFileDialog1 As OpenFileDialog
    Friend WithEvents GB_Smails As GroupBox
    Friend WithEvents smile_fon As Button
    Friend WithEvents ToolStripMenuItem1 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem2 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem4 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem5 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem6 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem7 As ToolStripMenuItem
    Friend WithEvents Check_ls As ToolStripMenuItem
    Friend WithEvents РазвлеченияToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents РадиоToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents PaintToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents ICQToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents btn_naps As Button
    Friend WithEvents Hint_User As ToolTip
    Friend WithEvents PictureBox16 As PictureBox
    Friend WithEvents PictureBox15 As PictureBox
    Friend WithEvents PictureBox14 As PictureBox
    Friend WithEvents PictureBox13 As PictureBox
    Friend WithEvents PictureBox12 As PictureBox
    Friend WithEvents PictureBox11 As PictureBox
    Friend WithEvents PictureBox10 As PictureBox
    Friend WithEvents PictureBox9 As PictureBox
    Friend WithEvents PictureBox8 As PictureBox
    Friend WithEvents PictureBox7 As PictureBox
    Friend WithEvents PictureBox6 As PictureBox
    Friend WithEvents PictureBox5 As PictureBox
    Friend WithEvents PictureBox4 As PictureBox
    Friend WithEvents PictureBox3 As PictureBox
    Friend WithEvents PictureBox2 As PictureBox
    Friend WithEvents PictureBox1 As PictureBox
    Friend WithEvents txtBoxHelper As TextBox
    Friend WithEvents ToolStripMenuItem3 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem8 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem9 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem10 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem11 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem12 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem13 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem14 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem15 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem16 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem17 As ToolStripMenuItem
    Friend WithEvents MenuStrip1 As MenuStrip
    Friend WithEvents ToolStripMenuItem18 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem20 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem23 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem24 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem25 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem28 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem19 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem21 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem22 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem26 As ToolStripMenuItem
    Friend WithEvents ToolStripMenuItem27 As ToolStripMenuItem
    Friend WithEvents РедактироватьПрофильToolStripMenuItem1 As ToolStripMenuItem
    Friend WithEvents ВыбратьШрифтToolStripMenuItem1 As ToolStripMenuItem
    Friend WithEvents ВыбратьЦветToolStripMenuItem1 As ToolStripMenuItem
    Friend WithEvents РадиоToolStripMenuItem1 As ToolStripMenuItem
    Friend WithEvents ЗаметкиToolStripMenuItem As ToolStripMenuItem
End Class
