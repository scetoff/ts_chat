<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()>
Partial Class frm_PrivateMessage
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_PrivateMessage))
        Me.pnl_lich = New System.Windows.Forms.Panel()
        Me.lbl_lich = New System.Windows.Forms.Label()
        Me.GB_lich = New System.Windows.Forms.GroupBox()
        Me.txt_text = New System.Windows.Forms.TextBox()
        Me.lbl_text = New System.Windows.Forms.Label()
        Me.txt_Komu = New System.Windows.Forms.TextBox()
        Me.lbl_Komu = New System.Windows.Forms.Label()
        Me.Cooldown_tmr = New System.Windows.Forms.Timer(Me.components)
        Me.pic_send = New System.Windows.Forms.PictureBox()
        Me.sendmes = New System.Windows.Forms.PictureBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.pnl_lich.SuspendLayout()
        Me.GB_lich.SuspendLayout()
        CType(Me.pic_send, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.sendmes, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'pnl_lich
        '
        Me.pnl_lich.BackColor = System.Drawing.Color.Black
        Me.pnl_lich.Controls.Add(Me.lbl_lich)
        Me.pnl_lich.ForeColor = System.Drawing.Color.White
        Me.pnl_lich.Location = New System.Drawing.Point(0, 35)
        Me.pnl_lich.Name = "pnl_lich"
        Me.pnl_lich.Size = New System.Drawing.Size(650, 36)
        Me.pnl_lich.TabIndex = 23
        '
        'lbl_lich
        '
        Me.lbl_lich.AutoSize = True
        Me.lbl_lich.BackColor = System.Drawing.Color.Black
        Me.lbl_lich.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_lich.ForeColor = System.Drawing.Color.White
        Me.lbl_lich.Location = New System.Drawing.Point(12, 4)
        Me.lbl_lich.Name = "lbl_lich"
        Me.lbl_lich.Size = New System.Drawing.Size(286, 26)
        Me.lbl_lich.TabIndex = 0
        Me.lbl_lich.Text = "Отправить личное сообщение"
        '
        'GB_lich
        '
        Me.GB_lich.BackColor = System.Drawing.Color.Black
        Me.GB_lich.Controls.Add(Me.pic_send)
        Me.GB_lich.Controls.Add(Me.sendmes)
        Me.GB_lich.Controls.Add(Me.txt_text)
        Me.GB_lich.Controls.Add(Me.lbl_text)
        Me.GB_lich.Controls.Add(Me.txt_Komu)
        Me.GB_lich.Controls.Add(Me.lbl_Komu)
        Me.GB_lich.ForeColor = System.Drawing.Color.White
        Me.GB_lich.Location = New System.Drawing.Point(12, 77)
        Me.GB_lich.Name = "GB_lich"
        Me.GB_lich.Size = New System.Drawing.Size(618, 341)
        Me.GB_lich.TabIndex = 0
        Me.GB_lich.TabStop = False
        '
        'txt_text
        '
        Me.txt_text.BackColor = System.Drawing.Color.White
        Me.txt_text.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_text.Location = New System.Drawing.Point(6, 119)
        Me.txt_text.MaxLength = 20
        Me.txt_text.Multiline = True
        Me.txt_text.Name = "txt_text"
        Me.txt_text.ScrollBars = System.Windows.Forms.ScrollBars.Vertical
        Me.txt_text.Size = New System.Drawing.Size(606, 159)
        Me.txt_text.TabIndex = 2
        '
        'lbl_text
        '
        Me.lbl_text.AutoSize = True
        Me.lbl_text.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_text.Location = New System.Drawing.Point(6, 90)
        Me.lbl_text.Name = "lbl_text"
        Me.lbl_text.Size = New System.Drawing.Size(169, 26)
        Me.lbl_text.TabIndex = 0
        Me.lbl_text.Text = "Текст сообщения"
        '
        'txt_Komu
        '
        Me.txt_Komu.BackColor = System.Drawing.Color.White
        Me.txt_Komu.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_Komu.Location = New System.Drawing.Point(6, 45)
        Me.txt_Komu.MaxLength = 40
        Me.txt_Komu.Name = "txt_Komu"
        Me.txt_Komu.Size = New System.Drawing.Size(606, 30)
        Me.txt_Komu.TabIndex = 1
        '
        'lbl_Komu
        '
        Me.lbl_Komu.AutoSize = True
        Me.lbl_Komu.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Komu.Location = New System.Drawing.Point(6, 16)
        Me.lbl_Komu.Name = "lbl_Komu"
        Me.lbl_Komu.Size = New System.Drawing.Size(165, 26)
        Me.lbl_Komu.TabIndex = 0
        Me.lbl_Komu.Text = "Тема сообщения"
        '
        'Cooldown_tmr
        '
        Me.Cooldown_tmr.Enabled = True
        Me.Cooldown_tmr.Interval = 1000
        '
        'pic_send
        '
        Me.pic_send.Image = Global.TSchat.My.Resources.Resources.plaplsw
        Me.pic_send.Location = New System.Drawing.Point(16, 300)
        Me.pic_send.Name = "pic_send"
        Me.pic_send.Size = New System.Drawing.Size(30, 20)
        Me.pic_send.TabIndex = 25
        Me.pic_send.TabStop = False
        Me.pic_send.Visible = False
        '
        'sendmes
        '
        Me.sendmes.Image = Global.TSchat.My.Resources.Resources.Send_mail
        Me.sendmes.Location = New System.Drawing.Point(562, 284)
        Me.sendmes.Name = "sendmes"
        Me.sendmes.Size = New System.Drawing.Size(50, 50)
        Me.sendmes.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.sendmes.TabIndex = 24
        Me.sendmes.TabStop = False
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(564, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 22
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(605, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 21
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(1, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 20
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(0, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(650, 35)
        Me.Stringer.TabIndex = 19
        Me.Stringer.TabStop = False
        '
        'frm_PrivateMessage
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(643, 430)
        Me.Controls.Add(Me.GB_lich)
        Me.Controls.Add(Me.pnl_lich)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.ForeColor = System.Drawing.Color.Black
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "frm_PrivateMessage"
        Me.Text = "Личные сообщения"
        Me.pnl_lich.ResumeLayout(False)
        Me.pnl_lich.PerformLayout()
        Me.GB_lich.ResumeLayout(False)
        Me.GB_lich.PerformLayout()
        CType(Me.pic_send, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.sendmes, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents Stringer As System.Windows.Forms.PictureBox
    Friend WithEvents pnl_lich As System.Windows.Forms.Panel
    Friend WithEvents lbl_lich As System.Windows.Forms.Label
    Friend WithEvents GB_lich As System.Windows.Forms.GroupBox
    Friend WithEvents txt_text As System.Windows.Forms.TextBox
    Friend WithEvents lbl_text As System.Windows.Forms.Label
    Friend WithEvents txt_Komu As System.Windows.Forms.TextBox
    Friend WithEvents lbl_Komu As System.Windows.Forms.Label
    Friend WithEvents sendmes As System.Windows.Forms.PictureBox
    Friend WithEvents pic_send As PictureBox
    Friend WithEvents Cooldown_tmr As Timer
End Class
