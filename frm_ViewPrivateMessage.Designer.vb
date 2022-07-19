<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_ViewPrivateMessage
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
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

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_ViewPrivateMessage))
        Me.trei = New System.Windows.Forms.PictureBox
        Me.EndIt = New System.Windows.Forms.PictureBox
        Me.ChatICO = New System.Windows.Forms.PictureBox
        Me.Stringer = New System.Windows.Forms.PictureBox
        Me.pnl_lich = New System.Windows.Forms.Panel
        Me.lbl_lich = New System.Windows.Forms.Label
        Me.msgcontext = New System.Windows.Forms.ContextMenuStrip(Me.components)
        Me.ОтправительToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem
        Me.Separator = New System.Windows.Forms.ToolStripSeparator
        Me.ПолучательToolStripMenuItem = New System.Windows.Forms.ToolStripMenuItem
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.pnl_lich.SuspendLayout()
        Me.msgcontext.SuspendLayout()
        Me.SuspendLayout()
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(601, 4)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 26
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(642, 4)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 25
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(0, 4)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 24
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(0, 4)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(689, 35)
        Me.Stringer.TabIndex = 23
        Me.Stringer.TabStop = False
        '
        'pnl_lich
        '
        Me.pnl_lich.BackColor = System.Drawing.Color.Linen
        Me.pnl_lich.Controls.Add(Me.lbl_lich)
        Me.pnl_lich.Location = New System.Drawing.Point(0, 37)
        Me.pnl_lich.Name = "pnl_lich"
        Me.pnl_lich.Size = New System.Drawing.Size(689, 36)
        Me.pnl_lich.TabIndex = 27
        '
        'lbl_lich
        '
        Me.lbl_lich.AutoSize = True
        Me.lbl_lich.BackColor = System.Drawing.Color.Linen
        Me.lbl_lich.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_lich.Location = New System.Drawing.Point(14, 5)
        Me.lbl_lich.Name = "lbl_lich"
        Me.lbl_lich.Size = New System.Drawing.Size(190, 26)
        Me.lbl_lich.TabIndex = 0
        Me.lbl_lich.Text = "Личные сообщения"
        '
        'msgcontext
        '
        Me.msgcontext.Items.AddRange(New System.Windows.Forms.ToolStripItem() {Me.ОтправительToolStripMenuItem, Me.Separator, Me.ПолучательToolStripMenuItem})
        Me.msgcontext.Name = "msgcontext"
        Me.msgcontext.RenderMode = System.Windows.Forms.ToolStripRenderMode.Professional
        Me.msgcontext.Size = New System.Drawing.Size(146, 54)
        '
        'ОтправительToolStripMenuItem
        '
        Me.ОтправительToolStripMenuItem.Name = "ОтправительToolStripMenuItem"
        Me.ОтправительToolStripMenuItem.Size = New System.Drawing.Size(145, 22)
        Me.ОтправительToolStripMenuItem.Text = "Отправитель"
        '
        'Separator
        '
        Me.Separator.Name = "Separator"
        Me.Separator.Size = New System.Drawing.Size(142, 6)
        '
        'ПолучательToolStripMenuItem
        '
        Me.ПолучательToolStripMenuItem.Name = "ПолучательToolStripMenuItem"
        Me.ПолучательToolStripMenuItem.Size = New System.Drawing.Size(145, 22)
        Me.ПолучательToolStripMenuItem.Text = "Получатель"
        '
        'frm_ViewPrivateMessage
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.SeaShell
        Me.ClientSize = New System.Drawing.Size(689, 517)
        Me.Controls.Add(Me.pnl_lich)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.Stringer)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.Name = "frm_ViewPrivateMessage"
        Me.Text = "ViewPrivateMessage"
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.pnl_lich.ResumeLayout(False)
        Me.pnl_lich.PerformLayout()
        Me.msgcontext.ResumeLayout(False)
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents Stringer As System.Windows.Forms.PictureBox
    Friend WithEvents pnl_lich As System.Windows.Forms.Panel
    Friend WithEvents lbl_lich As System.Windows.Forms.Label
    Friend WithEvents msgcontext As System.Windows.Forms.ContextMenuStrip
    Friend WithEvents ОтправительToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
    Friend WithEvents Separator As System.Windows.Forms.ToolStripSeparator
    Friend WithEvents ПолучательToolStripMenuItem As System.Windows.Forms.ToolStripMenuItem
End Class
