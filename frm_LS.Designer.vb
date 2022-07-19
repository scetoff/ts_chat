<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_LS
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
        Me.components = New System.ComponentModel.Container()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_LS))
        Me.lstmsgs = New System.Windows.Forms.ListView()
        Me.cluser = CType(New System.Windows.Forms.ColumnHeader(), System.Windows.Forms.ColumnHeader)
        Me.clsub = CType(New System.Windows.Forms.ColumnHeader(), System.Windows.Forms.ColumnHeader)
        Me.delmsg = New System.Windows.Forms.PictureBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.Hint_User = New System.Windows.Forms.ToolTip(Me.components)
        CType(Me.delmsg, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'lstmsgs
        '
        Me.lstmsgs.BackColor = System.Drawing.Color.Black
        Me.lstmsgs.BorderStyle = System.Windows.Forms.BorderStyle.None
        Me.lstmsgs.Columns.AddRange(New System.Windows.Forms.ColumnHeader() {Me.cluser, Me.clsub})
        Me.lstmsgs.Font = New System.Drawing.Font("Arial", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(0, Byte))
        Me.lstmsgs.ForeColor = System.Drawing.Color.White
        Me.lstmsgs.FullRowSelect = True
        Me.lstmsgs.HideSelection = False
        Me.lstmsgs.Location = New System.Drawing.Point(4, 41)
        Me.lstmsgs.Name = "lstmsgs"
        Me.lstmsgs.Size = New System.Drawing.Size(596, 308)
        Me.lstmsgs.TabIndex = 1
        Me.lstmsgs.UseCompatibleStateImageBehavior = False
        Me.lstmsgs.View = System.Windows.Forms.View.Details
        '
        'cluser
        '
        Me.cluser.Text = "Отправитель"
        Me.cluser.Width = 130
        '
        'clsub
        '
        Me.clsub.Text = "Тема"
        Me.clsub.Width = 500
        '
        'delmsg
        '
        Me.delmsg.BackColor = System.Drawing.Color.Transparent
        Me.delmsg.Image = Global.TSchat.My.Resources.Resources.trash
        Me.delmsg.Location = New System.Drawing.Point(557, 306)
        Me.delmsg.Name = "delmsg"
        Me.delmsg.Size = New System.Drawing.Size(35, 35)
        Me.delmsg.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.delmsg.TabIndex = 23
        Me.delmsg.TabStop = False
        Me.Hint_User.SetToolTip(Me.delmsg, "Удаляет выбранное сообщение")
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(519, 0)
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
        Me.EndIt.Location = New System.Drawing.Point(560, 0)
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
        Me.ChatICO.Location = New System.Drawing.Point(0, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 20
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-4, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(614, 35)
        Me.Stringer.TabIndex = 19
        Me.Stringer.TabStop = False
        '
        'frm_LS
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(604, 353)
        Me.Controls.Add(Me.delmsg)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.Controls.Add(Me.lstmsgs)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.Name = "frm_LS"
        Me.Text = "Личные сообщения"
        CType(Me.delmsg, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents cluser As ColumnHeader
    Friend WithEvents clsub As ColumnHeader
    Friend WithEvents lstmsgs As ListView
    Friend WithEvents trei As PictureBox
    Friend WithEvents EndIt As PictureBox
    Friend WithEvents ChatICO As PictureBox
    Friend WithEvents Stringer As PictureBox
    Friend WithEvents delmsg As PictureBox
    Friend WithEvents Hint_User As ToolTip
End Class
