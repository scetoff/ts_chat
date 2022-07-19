<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_paint
    Inherits System.Windows.Forms.Form

    'Форма переопределяет dispose для очистки списка компонентов.
    <System.Diagnostics.DebuggerNonUserCode()> _
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Является обязательной для конструктора форм Windows Forms
    Private components As System.ComponentModel.IContainer

    'Примечание: следующая процедура является обязательной для конструктора форм Windows Forms
    'Для ее изменения используйте конструктор форм Windows Form.  
    'Не изменяйте ее в редакторе исходного кода.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_paint))
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.image_box = New System.Windows.Forms.PictureBox()
        Me.Menu_StripLine = New System.Windows.Forms.MenuStrip()
        Me.ФайлToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.СохранитьToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ЗаливкаToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem()
        Me.ColorDialog1 = New System.Windows.Forms.ColorDialog()
        Me.btn_color = New System.Windows.Forms.Button()
        Me.SaveFileDialog1 = New System.Windows.Forms.SaveFileDialog()
        Me.cbox_size = New System.Windows.Forms.ComboBox()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.image_box, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.Menu_StripLine.SuspendLayout()
        Me.SuspendLayout()
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(191, -1)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 12
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(232, -1)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 11
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(-1, -1)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 10
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-1, -1)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(276, 35)
        Me.Stringer.TabIndex = 9
        Me.Stringer.TabStop = False
        '
        'image_box
        '
        Me.image_box.BackColor = System.Drawing.Color.Silver
        Me.image_box.Location = New System.Drawing.Point(-1, 63)
        Me.image_box.Name = "image_box"
        Me.image_box.Size = New System.Drawing.Size(268, 286)
        Me.image_box.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.image_box.TabIndex = 13
        Me.image_box.TabStop = False
        '
        'Menu_StripLine
        '
        Me.Menu_StripLine.AutoSize = False
        Me.Menu_StripLine.BackgroundImage = Global.TSchat.My.Resources.Resources.black
        Me.Menu_StripLine.Dock = System.Windows.Forms.DockStyle.None
        Me.Menu_StripLine.Font = New System.Drawing.Font("Comic Sans MS", 9.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Menu_StripLine.Items.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ФайлToolStripMenuItem, Me.ЗаливкаToolStripMenuItem})
        Me.Menu_StripLine.Location = New System.Drawing.Point(-1, 34)
        Me.Menu_StripLine.Name = "Menu_StripLine"
        Me.Menu_StripLine.RenderMode = System.Windows.Forms.ToolStripRenderMode.Professional
        Me.Menu_StripLine.Size = New System.Drawing.Size(276, 26)
        Me.Menu_StripLine.TabIndex = 14
        '
        'ФайлToolStripMenuItem
        '
        Me.ФайлToolStripMenuItem.DropDownItems.AddRange(New System.Windows.Forms.ToolStripItem() {Me.СохранитьToolStripMenuItem})
        Me.ФайлToolStripMenuItem.Font = New System.Drawing.Font("Comic Sans MS", 9.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.ФайлToolStripMenuItem.ForeColor = System.Drawing.Color.White
        Me.ФайлToolStripMenuItem.Name = "ФайлToolStripMenuItem"
        Me.ФайлToolStripMenuItem.Size = New System.Drawing.Size(50, 22)
        Me.ФайлToolStripMenuItem.Text = "Файл"
        '
        'СохранитьToolStripMenuItem
        '
        Me.СохранитьToolStripMenuItem.Image = CType(resources.GetObject("СохранитьToolStripMenuItem.Image"), System.Drawing.Image)
        Me.СохранитьToolStripMenuItem.ImageTransparentColor = System.Drawing.Color.Magenta
        Me.СохранитьToolStripMenuItem.Name = "СохранитьToolStripMenuItem"
        Me.СохранитьToolStripMenuItem.ShortcutKeys = CType((System.Windows.Forms.Keys.Control Or System.Windows.Forms.Keys.S), System.Windows.Forms.Keys)
        Me.СохранитьToolStripMenuItem.Size = New System.Drawing.Size(181, 22)
        Me.СохранитьToolStripMenuItem.Text = "Сохранить"
        '
        'ЗаливкаToolStripMenuItem
        '
        Me.ЗаливкаToolStripMenuItem.Font = New System.Drawing.Font("Comic Sans MS", 9.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.ЗаливкаToolStripMenuItem.ForeColor = System.Drawing.Color.White
        Me.ЗаливкаToolStripMenuItem.Name = "ЗаливкаToolStripMenuItem"
        Me.ЗаливкаToolStripMenuItem.Size = New System.Drawing.Size(67, 22)
        Me.ЗаливкаToolStripMenuItem.Text = "Заливка"
        '
        'btn_color
        '
        Me.btn_color.BackColor = System.Drawing.Color.Black
        Me.btn_color.BackgroundImage = Global.TSchat.My.Resources.Resources.rainbow
        Me.btn_color.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch
        Me.btn_color.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.btn_color.ForeColor = System.Drawing.Color.Black
        Me.btn_color.Location = New System.Drawing.Point(240, 35)
        Me.btn_color.Name = "btn_color"
        Me.btn_color.Size = New System.Drawing.Size(24, 24)
        Me.btn_color.TabIndex = 15
        Me.btn_color.UseVisualStyleBackColor = False
        '
        'cbox_size
        '
        Me.cbox_size.BackColor = System.Drawing.Color.White
        Me.cbox_size.ForeColor = System.Drawing.Color.Black
        Me.cbox_size.FormattingEnabled = True
        Me.cbox_size.Items.AddRange(New Object() {"14", "16", "18", "20", "32", "36", "48", "72"})
        Me.cbox_size.Location = New System.Drawing.Point(183, 37)
        Me.cbox_size.Name = "cbox_size"
        Me.cbox_size.Size = New System.Drawing.Size(48, 21)
        Me.cbox_size.TabIndex = 17
        '
        'frm_paint
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(267, 350)
        Me.Controls.Add(Me.cbox_size)
        Me.Controls.Add(Me.btn_color)
        Me.Controls.Add(Me.image_box)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.Controls.Add(Me.Menu_StripLine)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.MainMenuStrip = Me.Menu_StripLine
        Me.Name = "frm_paint"
        Me.Text = "frm_paint"
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.image_box, System.ComponentModel.ISupportInitialize).EndInit()
        Me.Menu_StripLine.ResumeLayout(False)
        Me.Menu_StripLine.PerformLayout()
        Me.ResumeLayout(False)

    End Sub

    Friend WithEvents trei As PictureBox
    Friend WithEvents EndIt As PictureBox
    Friend WithEvents ChatICO As PictureBox
    Friend WithEvents Stringer As PictureBox
    Friend WithEvents image_box As PictureBox
    Friend WithEvents Menu_StripLine As MenuStrip
    Friend WithEvents ФайлToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents СохранитьToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents ColorDialog1 As ColorDialog
    Friend WithEvents btn_color As Button
    Friend WithEvents ЗаливкаToolStripMenuItem As ToolStripMenuItem
    Friend WithEvents SaveFileDialog1 As SaveFileDialog
    Friend WithEvents cbox_size As ComboBox
End Class
